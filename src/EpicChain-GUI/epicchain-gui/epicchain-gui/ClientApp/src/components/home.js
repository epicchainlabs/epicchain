/* eslint-disable */
import React, { Component } from "react";
import { Link } from "react-router-dom";
import { Layout, Row, Col, Typography } from "antd";

import Sync from "./sync";
import "../static/css/site.css";
import "../static/css/home.css";
import img from "../static/images/logo.svg";
import blockimg from "../static/images/1.svg";
import aniblockimg from "../static/images/1-ani.svg";
import walletimg from "../static/images/2.svg";
import aniwalletimg from "../static/images/2-ani.svg";
import contractimg from "../static/images/3.svg";
import anicontractimg from "../static/images/3-ani.svg";
import adavancedimg from "../static/images/4.svg";
import aniadavancedimg from "../static/images/4-ani.svg";
import { withTranslation } from "react-i18next";

const { Text } = Typography;
const { Content } = Layout;

@withTranslation()
class Home extends Component {
  state = {
    visible: false,
    confirmLoading: false,
  };
  render() {
    const { t } = this.props;
    return (
      <div>
        <Layout className="home-content">
          <Content>
            <Sync></Sync>
            <div className="pv1 text-c">
              <img src={img} className="app-logo" alt="img" />
            </div>
          </Content>
          <Content className="home-icon">
            <Row gutter={60}>
              <Col span={6}>
                <Link to="/chain">
                  <div className="home-link">
                    <img className="show-img" src={blockimg} alt="img" />
                    <img className="hidden-img" src={aniblockimg} alt="img" />
                    <span>{t("home.blockchain")}</span>
                  </div>
                </Link>
              </Col>
              <Col span={6}>
                <Link to="/wallet/walletlist">
                  <div className="home-link">
                    <img className="show-img" src={walletimg} alt="img" />
                    <img className="hidden-img" src={aniwalletimg} alt="img" />
                    <span>{t("home.wallet")}</span>
                  </div>
                </Link>
              </Col>
              <Col span={6}>
                <Link to="/contract">
                  <div className="home-link">
                    <img className="show-img" src={contractimg} alt="img" />
                    <img
                      className="hidden-img"
                      src={anicontractimg}
                      alt="img"
                    />
                    <span>{t("home.contract")}</span>
                  </div>
                </Link>
              </Col>
              <Col span={6}>
                <Link to="/advanced">
                  <div className="home-link">
                    <img className="show-img" src={adavancedimg} alt="img" />
                    <img
                      className="hidden-img"
                      src={aniadavancedimg}
                      alt="img"
                    />
                    <span>{t("home.advanced")}</span>
                  </div>
                </Link>
              </Col>
            </Row>
          </Content>
        </Layout>
      </div>
    );
  }
}

export default Home;
