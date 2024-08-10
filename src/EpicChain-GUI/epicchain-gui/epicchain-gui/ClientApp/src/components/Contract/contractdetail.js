/* eslint-disable */
import React from "react";
import { Layout, Row, Col, message, PageHeader, Table, Tag } from "antd";
import Sync from "../sync";
import post from "../../core/request";
import { withTranslation } from "react-i18next";

const { Content } = Layout;
const { Column } = Table;

@withTranslation()
class Contractdetail extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      detail: {},
      hash: "",
      storage: false,
      payable: false,
      showMain: false,
    };
  }
  componentDidMount() {
    let _hash = location.pathname.split(":").pop();
    this.contractDetail(_hash, (res) => {
      let _methodarr = new Array();
      _methodarr = res.manifest.abi.methods ? res.manifest.abi.methods : [];
      _methodarr.map((item, index) => {
        item.key = index;
      });

      let _eventarr = new Array();
      _eventarr = res.manifest.abi.events ? res.manifest.abi.events : [];
      _eventarr.map((item, index) => {
        item.key = index;
      });

      this.setState({
        hash: _hash,
        detail: res,
        storage: res.hasStorage,
        payable: res.payable,
        methodarr: _methodarr,
        eventarr: _eventarr,
      });
    });
  }
  contractDetail = (hash, callback) => {
    const { t } = this.props;
    let params = { contractHash: hash };
    post("GetContract", params)
      .then(function (response) {
        var _data = response.data;
        if (_data.msgType === -1) {
          message.info(t("contract.hash fail"));
          return;
        } else if (_data.msgType === 3) {
          callback(_data.result);
        }
      })
      .catch(function (error) {
        console.log(error);
        console.log("error");
      });
  };
  render() {
    const { detail, methodarr, eventarr, storage, payable } = this.state;
    const { t } = this.props;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row className="mb2" gutter={[30, 0]} type="flex">
            <Col span={24} className="bg-white pv4">
              <PageHeader title={t("contract.contract detail")}></PageHeader>
              <div className="info-detail pv3">
                <div className="f-1 pa3">
                  <span>{t("contract.script-hash")}: &nbsp;&nbsp;&nbsp;</span>
                  {detail.contractHash}
                  {storage ? (
                    <Tag className="ml3 ant-tag-green">storage</Tag>
                  ) : null}
                  {payable ? (
                    <Tag className="ml3 ant-tag-green">payable</Tag>
                  ) : null}
                </div>
                <Row>
                  <Col span={12}>
                    <ul className="detail-ul">
                      <li>
                        <span className="hint">
                          {t("common.contract")} ID：
                        </span>
                        {detail.contractId}
                      </li>
                    </ul>
                  </Col>
                </Row>
              </div>

              <Table
                dataSource={methodarr}
                pagination={false}
                bordered={true}
                title={() => t("contract.method")}
              >
                <Column
                  title={t("contract.name")}
                  dataIndex="name"
                  key="name"
                  width={150}
                />
                <Column
                  title={t("contract.para")}
                  dataIndex="parameters"
                  key="parameters"
                  render={(parameters, index) =>
                    parameters.map((item, index) => (
                      <span className="para-tag" key={index}>
                        {item.type}
                        <em>{item.name}</em>
                      </span>
                    ))
                  }
                />
                <Column
                  title={t("contract.return")}
                  dataIndex="returnType"
                  key="returnType"
                  width={100}
                  render={(returnType) =>
                    returnType ? (
                      <span className="para-tag">{returnType}</span>
                    ) : null
                  }
                />
              </Table>
              <Table
                className="mt3 mb2"
                dataSource={eventarr}
                pagination={false}
                bordered={true}
                title={() => t("contract.notify")}
              >
                <Column
                  title={t("contract.name")}
                  dataIndex="name"
                  key="name"
                  width={150}
                />
                <Column
                  title={t("contract.para")}
                  dataIndex="parameters"
                  key="parameters"
                  render={(parameters, index) =>
                    parameters.map((item, index) => (
                      <span className="para-tag" key={index}>
                        {item.type}
                        <em>{item.name}</em>
                      </span>
                    ))
                  }
                />
                <Column
                  title={t("contract.return")}
                  dataIndex="returnType"
                  key="returnType"
                  width={100}
                  render={(returnType) =>
                    returnType ? (
                      <span className="para-tag">{returnType}</span>
                    ) : null
                  }
                />
              </Table>
            </Col>
          </Row>
        </Content>
      </Layout>
    );
  }
}

export default Contractdetail;
