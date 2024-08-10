/* eslint-disable */
import React, { useState } from "react";
import "antd/dist/antd.min.css";
import {
  Input,
  Icon,
  PageHeader,
  Modal,
  Select,
  Row,
  Col,
  Form,
  message,
  Button,
  Layout,
} from "antd";
import "../../static/css/wallet.css";
import Sync from "../sync";
import { FolderOpenOutlined, SwapOutlined } from "@ant-design/icons";
import { observer, inject } from "mobx-react";
import { withTranslation } from "react-i18next";
import { app, dialog } from '@electron/remote';
import { postAsync } from "../../core/request";
import { withAuthenticated } from '../../core/authentication';
import fs from "fs";


const { Content } = Layout;
const { TextArea } = Input;


@withAuthenticated
@withTranslation()
@inject("walletStore")
@observer
class ContractUpgrade extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      showInputs: false,
      manipath: "",
      nefpath: "",
      disabled: true,
      visible: false,
      isOpenDialog: false,
    };
  }
  selectNef = async () => {
    this.opendialog("nef", async (res) => {
      let nef = res.filePaths[0];
      if (nef) {
        let manifest = nef.substring(0, nef.length - 3) + "manifest.json";
        if (fs.existsSync(manifest)) {
          await this.setState({ manipath: manifest });
        }
        await this.setState({ nefpath: nef, isOpenDialog: false });
        this.onFill()
      }
    });
  };
  selectMani = async () => {
    this.opendialog("json", async (res) => {
      await this.setState({ manipath: res.filePaths[0], isOpenDialog: false });
      this.onFill();
    });
  };
  browseDialog = () => {
    const { isOpenDialog } = this.state;
    return isOpenDialog;
  };
  opendialog = (str, callback) => {
    if (this.browseDialog()) return;
    const { t } = this.props;
    str = str || "";
    this.setState({ disabled: true, isOpenDialog: true });
    dialog
      .showOpenDialog({
        title: t("contract.select {file} path title", { file: str }),
        defaultPath: "/",
        filters: [
          {
            name: "*",
            extensions: [str],
          },
        ],
      })
      .then(function (res) {
        callback(res);
      })
      .catch(function (error) {
        console.log(error);
      });
  };
  onFill = () => {
    this.Form.setFieldsValue({
      nefPath: this.state.nefpath,
      manifestPath: this.state.manipath,
      tresult: this.state.tresult,
    });
  };

  onResetForm = (hash, msg) => {
    this.setState({ showInputs: false });
    this.Form.resetFields();
    if (hash) {
      this.Form.setFieldsValue({
        contractHash: hash
      });
    }
    if (msg) {
      message.info(msg);
    }
  }

  checkContract = () => {
    const { t } = this.props;
    let _hash = this.ContractHash.input.value.trim();
    if (!_hash) {
      message.info(t("contract.search input check"));
      return;
    }
    var params = { contractHash: _hash };
    postAsync("GetContract", params).then((data) => {
      if (data.msgType < 0) {
        this.onResetForm(_hash, t("contract.search fail"))
        return;
      }
      let updateMethodIndex = data.result.manifest.abi.methods.findIndex(item => {
        return item.name === "update" && item.returnType === "Void" && item.parameters.length === 3;
      });
      if (data.result.contractId < 0 || updateMethodIndex < 0) {
        this.onResetForm(_hash, t("contract.contract can not update"))
        return;
      }
      this.setState({ showInputs: true });
    }).catch((error) => {
      console.log(error);
    });
  };

  updateContract = (params) => {
    return postAsync("UpdateContract", params);
  };

  testUpdate = () => {
    const { t } = this.props;
    this.Form
      .validateFields()
      .then(async (params) => {
        params.sendTx = false;
        let data = await this.updateContract(params);
        if (data.msgType < 0) {
          let res = data.error;
          Modal.error({
            title: t("contract.fail title"),
            width: 400,
            content: (
              <div className="show-pri">
                <p>
                  {t("error code")}: {res.code}
                </p>
                <p>
                  {t("error msg")}: {res.message}
                </p>
              </div>
            ),
            okText: t("button.ok"),
          });
          return;
        }
        await this.setState({ disabled: false, tresult: JSON.stringify(data.result) });
        this.onFill()
      })
      .catch(function () {
        message.error(t("contract.please select file path"));
      });
  };

  submitUpdate = () => {
    const { t } = this.props;
    this.Form
      .validateFields()
      .then(async (params) => {
        params.sendTx = true;
        let data = await this.updateContract(params);
        if (data.msgType < 0) {
          let res = data.error;
          message.error(res.message);
          return;
        }
        let result = data.result;
        Modal.success({
          title: t("contract.deploy success"),
          width: 650,
          content: (
            <div className="show-pri">
              <p>TxID: {result?.txId ? result.txId : "--"}</p>
              <p>
                ScrptHash:{" "}
                {result.contractHash ? result.contractHash : "--"}
              </p>
              <p>Gas: {result.gasConsumed ? result.gasConsumed : "--"}</p>
            </div>
          ),
          okText: t("button.ok"),
        });
      })
      .catch(function () {
        message.error(t("contract.please select file path"));
      });
  }
  render() {
    const { t } = this.props;
    const accounts = this.props.walletStore.accountlist;

    const { disabled } = this.state;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row
            gutter={[30, 0]}
            className="bg-white pv4"
            style={{ minHeight: "calc( 100vh - 150px )" }} >
            <Col span={24}>
              <PageHeader title={t("contract.upgrade contract")}></PageHeader>

              <Form className="trans-form" ref={form => this.Form = form} onFinish={this.submitUpdate} >
                <Row className="mt3 mb5">
                  <Col span={20}>
                    <Form.Item name="contractHash"
                      label={t("contract.scripthash")}
                      rules={[
                        {
                          required: true,
                          message: t("contract.search fail"),
                        },
                      ]}>
                      <Input ref={input => this.ContractHash = input} placeholder="Scripthash" />
                    </Form.Item>

                    {this.state.showInputs ?
                      <div>
                        <Form.Item
                          name="nefPath"
                          label="Neo Executable File (.nef)"
                          onClick={this.selectNef}
                          rules={[
                            {
                              required: true,
                              message: t("contract.please select file path"),
                            },
                          ]}
                        >
                          <Input
                            className="dis-file"
                            placeholder={t("select file")}
                            disabled
                            suffix={<FolderOpenOutlined />}
                          />
                        </Form.Item>
                        <Form.Item
                          name="manifestPath"
                          label="Neo Contract Manifest (.manifest.json)"
                          onClick={this.selectMani}
                          rules={[
                            {
                              required: true,
                              message: t("contract.please select file path"),
                            },
                          ]}
                        >
                          <Input
                            className="dis-file"
                            placeholder={t("select file")}
                            disabled
                            suffix={<FolderOpenOutlined />}
                          />
                        </Form.Item>

                        <Form.Item
                          name="cosigners"
                          label={t("contract.cosigners")}>
                          <Select
                            placeholder={t("contract.choose account")}
                            mode="tags"
                            style={{ width: '100%' }}>
                            {accounts.map((item) => {
                              return (
                                <Option key={item.address}>{item.address}</Option>
                              )
                            })}
                          </Select>
                        </Form.Item>
                      </div>
                      : null}

                  </Col>
                  <Col span={4}>
                    <Button className="w200 form-btn" onClick={this.checkContract} loading={this.state.loading}>{t("button.search")}</Button>
                  </Col>
                </Row>

                <Form.Item className="text-c w200" >
                  <Button type="primary" htmlType="button" onClick={this.testUpdate}>
                    {t('button.test invoke')}
                  </Button>
                </Form.Item>
                <div className="pa3 mb5">
                  <p className="mb5 bolder">{t("contract.test result")}</p>
                  <TextArea rows={3} value={this.state.tresult} />
                </div>

                <Form.Item className="text-c w200">
                  <Button
                    className="mt3"
                    type="primary"
                    htmlType="submit"
                    disabled={disabled}
                    loading={this.state.iconLoading}
                  >
                    {t("button.send")}
                  </Button>
                </Form.Item>
              </Form>
            </Col>


          </Row>
        </Content>
      </Layout>
    );
  };
}

export default ContractUpgrade;
