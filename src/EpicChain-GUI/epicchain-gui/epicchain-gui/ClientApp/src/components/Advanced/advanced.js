import React from "react";
import { PageHeader, Button, Row, Col, Layout } from "antd";
import Sync from "../sync";
import "../../static/css/advanced.css";
import Datatrans from "../Common/datatrans";
import { SwapOutlined, PaperClipOutlined } from "@ant-design/icons";
import { withTranslation } from "react-i18next";
import { shell } from "electron";

const { Content } = Layout;

@withTranslation()
class Advanced extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      visible: false,
    };
  }
  showDrawer = () => {
    this.setState({
      visible: true,
    });
  };
  onClose = () => {
    this.setState({
      visible: false,
    });
  };
  openUrl(url) {
    return () => {
      shell.openExternal(url);
    };
  }
  render() {
    const { t } = this.props;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row gutter={[30, 0]} style={{ minHeight: "calc( 100vh - 120px )" }}>
            <Col span={24} className="bg-white pv4">
              <PageHeader title={"GUI " + t("advanced.tools")}></PageHeader>
              <Row className="mt3" gutter={[30, 0]}>
                <Col span={6}>
                  <Button
                    className="ml3 pa2"
                    type="primary"
                    onClick={this.showDrawer}
                  >
                    <SwapOutlined /> {t("advanced.data trans")}
                  </Button>
                </Col>
              </Row>
              <PageHeader
                className="mt2"
                title={t("advanced.dev tools")}
              ></PageHeader>
              <Row className="mt3" gutter={[30, 0]}>
                <Col span={6}>
                  <a
                    className="ml3"
                    onClick={this.openUrl("https://n3t5wish.ngd.network/")}
                  >
                    <PaperClipOutlined /> {t("advanced.test coin")}
                  </a>
                </Col>
                <Col span={6}>
                  <a
                    className="ml3"
                    onClick={this.openUrl("https://docs.neo.org/v3")}
                  >
                    <PaperClipOutlined /> {t("advanced.dev docs")}
                  </a>
                </Col>
                <Col span={6}>
                  <a
                    className="ml3"
                    onClick={this.openUrl("https://neo.org/dev")}
                  >
                    <PaperClipOutlined /> {t("advanced.more")}
                  </a>
                </Col>
              </Row>
            </Col>
          </Row>
          <Datatrans visible={this.state.visible} onClose={this.onClose} />
        </Content>
      </Layout>
    );
  }
}

export default Advanced;
