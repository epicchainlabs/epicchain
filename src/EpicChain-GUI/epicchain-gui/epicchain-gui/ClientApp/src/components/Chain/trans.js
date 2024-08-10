import React from "react";
import { Layout, Row, Col, Tabs } from "antd";
import Transaction from "../Transaction/transaction";
import Untransaction from "../Transaction/untransaction";
import Sync from "../sync";
import { withTranslation } from "react-i18next";

const { Content } = Layout;

@withTranslation()
class Blocktrans extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      size: "default",
      translist: [],
    };
  }
  render() {
    const { t } = this.props;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row gutter={[30, 0]} className="mb1">
            <Col span={24} className="bg-white pv4">
              <Tabs
                className="tran-title trans-list-title"
                defaultActiveKey="1"
                items={[
                  { label: t("blockchain.transactions"), key: 1, children: (<Transaction content={t("blockchain.transactions")} page="all" />) },
                  { label: t("blockchain.transaction.pending"), key: 2, children: (<Untransaction content={t("blockchain.transaction.pending")} />) }
                ]}
              >
              </Tabs>
            </Col>
          </Row>
        </Content>
      </Layout>
    );
  };
}

export default Blocktrans;
