/* eslint-disable */
import React from "react";
import "antd/dist/antd.min.css";
import {
  Checkbox,
  PageHeader,
  Modal,
  Alert,
  Row,
  Col,
  Form,
  Select,
  Button,
  message,
} from "antd";
import { Layout } from "antd";
import Sync from "../sync";
import { withTranslation } from "react-i18next";
import "../../static/css/advanced.css";
import { postAsync } from "../../core/request";
import { withAuthenticated } from '../../core/authentication';
const { Option } = Select;

const CheckboxGroup = Checkbox.Group;
const { Content } = Layout;

@withTranslation()
@withAuthenticated
class Advancedcandidate extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      size: "default",
      accountlist: [],
    };
  }
  componentDidMount() {
    this.listPublicKey((res) => {
      this.setState({
        accountlist: res.result,
      });
    });
  }
  listPublicKey = async (callback) => {
    const { t } = this.props;
    let response = await postAsync("ListCandidatePublicKey");
    if (response.msgType === -1) {
      let res = response.error;
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
    callback(response);
  };
  onCandidate = async (fieldsValue) => {
    const { t } = this.props;
    let response = await postAsync("ApplyForValidator", {
      pubkey: fieldsValue.pubkey,
    });
    if (response.msgType === -1) {
      let res = response.error;
      Modal.error({
        title: t("advanced.candidate fail"),
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

    Modal.success({
      title: t("advanced.candidate success"),
      width: 400,
      content: (
        <div className="show-pri">
          <p>TxID : {response.result.txId ? response.result.txId : "--"}</p>
        </div>
      ),
      okText: t("button.ok"),
    });
  };
  render() {
    const { t } = this.props;
    const { disabled, accountlist } = this.state;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row gutter={[30, 0]} style={{ minHeight: "calc( 100vh - 120px )" }}>
            <Col span={24} className="bg-white pv4">
              <PageHeader title={t("advanced.candidate")}></PageHeader>

              <div className="pa3">
                <Alert
                  className="mt3 mb3"
                  type="warning"
                  message={
                    <div>
                      <p className="bolder mb5">{t("advanced.candidate")}</p>
                      <p className="mb5 font-s">
                        {t("advanced.candidate info")}
                      </p>
                      <ul className="list-num">
                        <li>{t("advanced.candidate step1")}</li>
                        <li>{t("advanced.candidate step2")}</li>
                        <li>{t("advanced.candidate step3")}</li>
                      </ul>
                    </div>
                  }
                  showIcon
                />
                <Form ref="formRef" onFinish={this.onCandidate}>
                  <h4 className="bolder mb4">{t("advanced.be candidate")}</h4>
                  <Form.Item
                    name="pubkey"
                    className="select-vote"
                    rules={[
                      {
                        required: true,
                        message: t("advanced.need address"),
                      },
                    ]}
                  >
                    <Select
                      placeholder={t("advanced.select address")}
                      style={{ width: "100%" }}
                      onChange={this.setAddress}
                    >
                      {accountlist.map((item) => {
                        return (
                          <Option key={item.publicKey}>{item.address}</Option>
                        );
                      })}
                    </Select>
                  </Form.Item>
                  <p className="text-c mt3">
                    <Button
                      type="primary"
                      htmlType="submit"
                      disabled={disabled}
                      loading={this.state.iconLoading}
                    >
                      {t("button.confirm")}
                    </Button>
                  </p>
                </Form>
              </div>
            </Col>
          </Row>
        </Content>
      </Layout>
    );
  };
}

export default Advancedcandidate;
