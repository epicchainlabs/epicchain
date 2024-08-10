/* eslint-disable */
import React from "react";
import "antd/dist/antd.min.css";
import "../../static/css/menu.css";
import "../../static/css/wallet.css";
import { Link } from "react-router-dom";
import { Layout, Row, Col, Button, Divider } from "antd";
import {
  Walletopen,
  Walletcreate,
  Walletprivate,
  Walletencrypted,
} from "./walletaction";
import Sync from "../sync";
import { withTranslation } from "react-i18next";
import { ArrowLeftOutlined, CloseOutlined } from "@ant-design/icons";

const { Footer } = Layout;

@withTranslation()
class Wallet extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      size: "default",
      iconLoading: false,
      showElem: true,
      children: "",
      path: "",
      login: false,
    };
  }
  changeTab() {
    this.setState((prevState) => ({
      showElem: !prevState.showElem,
    }));
  }
  getInset = (ele) => {
    return () => {
      this.setState({ showElem: false });
      switch (ele) {
        case 0:
          this.setState({ children: <Walletopen /> });
          break;
        case 1:
          this.setState({ children: <Walletcreate /> });
          break;
        case 2:
          this.setState({ children: <Walletprivate /> });
          break;
        case 3:
          this.setState({ children: <Walletencrypted /> });
          break;
        case 4:
          this.setState({ children: <Walletopen /> });
          break;
        default:
          this.setState({ showElem: true });
          break;
      }
    };
  };
  render() {
    const { t } = this.props;
    return (
      <Layout className="gui-container">
        <Sync />
        <div className="wa-content mt2">
          <div className="wa-link">
            {/* 设置一个显示值及返回路径 */}
            {!this.state.showElem ? (
              <a className="back" onClick={this.getInset(-1)} key="1">
                <ArrowLeftOutlined />
              </a>
            ) : null}
            <Link className="close" to="/">
              <CloseOutlined />
            </Link>
          </div>
          <div className="logo mt2 mb1"></div>
          <div className="wa-open">
            {this.state.showElem ? (
              <div>
                <Button type="primary" onClick={this.getInset(0)}>
                  {t("wallet.open wallet file")}
                </Button>
                <Button
                  className="mt3 mb2"
                  type="primary"
                  onClick={this.getInset(1)}
                >
                  {t("wallet.create wallet file")}
                </Button>

                <Divider className="t-light">
                  {t("wallet.import wallet")}
                </Divider>
                <Row justify="space-between">
                  <Col span={6}>
                    <Button size="small" onClick={this.getInset(2)}>
                      {t("wallet.private key")}
                    </Button>
                  </Col>
                  <Col span={6} offset={3}>
                    <Button size="small" onClick={this.getInset(3)}>
                      {t("wallet.Nep2 key")}
                    </Button>
                  </Col>
                  <Col span={6} offset={3}>
                    <Button size="small" disabled>
                      {t("wallet.mnemonic")}
                    </Button>
                  </Col>
                </Row>
              </div>
            ) : null}
            {!this.state.showElem ? <div>{this.state.children}</div> : null}
          </div>
        </div>
            <Footer className="mt1">Copyright © 2015-2024 The Neo Project.</Footer>
      </Layout>
    );
  };
}

export default Wallet;
