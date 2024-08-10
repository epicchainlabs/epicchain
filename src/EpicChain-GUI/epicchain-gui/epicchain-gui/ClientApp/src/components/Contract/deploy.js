/* eslint-disable */
import React, { useState, createRef } from "react";
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
} from "antd";
import { Layout } from "antd";
import "../../static/css/wallet.css";
import Sync from "../sync";
import { FolderOpenOutlined } from "@ant-design/icons";
import { withTranslation } from "react-i18next";
import { app, dialog } from '@electron/remote';
import fs from "fs";
import { postAsync } from "../../core/request";
import { withAuthenticated } from '../../core/authentication';

const { Content } = Layout;
const { TextArea } = Input;

@withTranslation()
@withAuthenticated
class Contractdeploy extends React.Component {
  constructor(props) {
    super(props);
    this.myForm = createRef();
    this.state = {
      size: "default",
      nefpath: "",
      manipath: "",
      disabled: true,
      visible: false,
      cost: -1,
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
    this.myForm.current.setFieldsValue({
      nefPath: this.state.nefpath,
      manifestPath: this.state.manipath,
      tresult: this.state.tresult,
    });
  };
  onTest = () => {
    const { t } = this.props;
    this.myForm.current
      .validateFields()
      .then((data) => {
        let _params = data;
        _params.sendTx = false;
        this.deployContract(_params, (res) => {
          this.setState(
            {
              disabled: false,
              tresult: JSON.stringify(res.result),
              cost: res.result.gasConsumed,
            },
            this.onFill()
          );
        });
      })
      .catch(function () {
        message.error(t("contract.please select file path"));
      });
  };
  ondeploy = (fieldsValue) => {
    const { t } = this.props;
    let _params = fieldsValue;
    _params.sendTx = true;
    this.deployContract(_params, (res) => {
      Modal.success({
        title: t("contract.deploy success"),
        width: 650,
        content: (
          <div className="show-pri">
            <p>TxID: {res.result.txId ? res.result.txId : "--"}</p>
            <p>
              ScrptHash:{" "}
              {res.result.contractHash ? res.result.contractHash : "--"}
            </p>
            <p>Gas: {res.result.gasConsumed ? res.result.gasConsumed : "--"}</p>
          </div>
        ),
        okText: t("button.ok"),
      });
      this.myForm.current.setFieldsValue({
        nefPath: "",
        manifestPath: "",
        tresult: "",
      });
    });
  };
  deployContract = (params, callback) => {
    const { t } = this.props;
    postAsync("DeployContract", params)
      .then((data) => {
        if (data.msgType === -1) {
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
        } else if (data.msgType === 3) {
          callback(data);
        }
      })
      .catch(function (error) {
        // console.log(error);
      });
  };
  render() {
    const { t } = this.props;
    const { disabled, cost } = this.state;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row gutter={[30, 0]}>
            <Col span={24} className="bg-white pv4">
              <PageHeader title={t("contract.deploy contract")}></PageHeader>
              <Form
                ref={this.myForm}
                className="trans-form mt3"
                onFinish={this.ondeploy}
              >
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
                <Form.Item className="text-c w200">
                  <Button
                    className="mt5"
                    type="primary"
                    htmlType="button"
                    onClick={this.onTest}
                  >
                    {t("button.test deploy")}
                  </Button>
                </Form.Item>
                <div className="pa3 mb5">
                  <p className="mb5 bolder">{t("contract.test result")}</p>
                  <TextArea rows={3} value={this.state.tresult} />
                </div>
                {/* {cost>=0?<p className="text-c small mt4 mb0">手续费：{cost} GAS</p>:null} */}
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

export default Contractdeploy;
