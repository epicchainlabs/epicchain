/* eslint-disable */
import React from "react";
import "antd/dist/antd.min.css";
import "../../static/css/menu.css";
import "../../static/css/wallet.css";
import { Layout, Menu, Icon } from "antd";
import { Link } from "react-router-dom";
import MenuDown from "../Common/menudown";
import { HomeOutlined, PartitionOutlined } from "@ant-design/icons";
import { withTranslation } from "react-i18next";

const { Sider } = Layout;

@withTranslation()
class Chainlayout extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      size: "default",
    };
  }
  toHome = () => {
    location.href = location.origin;
  };
  render() {
    const { t } = this.props;
    const menuItems = [
      {
        label: (
          <Link to="/">
            <HomeOutlined />
            {t("sideBar.home")}
          </Link>),
        key: "item-home"
      },
      {
        label: (
          <span>
            <PartitionOutlined />
            <span>{t("sideBar.blockchain")}</span>
          </span>),
        key: "sub1",
        children: [
          { label: (<Link to="/chain">{t("sideBar.blocks")}</Link>), key: 1 },
          { label: (<Link to="/chain/transaction">{t("sideBar.transactions")}</Link>), key: 2 },
          { label: (<Link to="/chain/asset">{t("sideBar.assets")}</Link>), key: 3 }

        ]
      }
    ];
    return (
      <div style={{ height: "100%" }}>
        <Sider className="menu-logo" style={{ height: "100%" }}>
          <Menu
            className="menu-scroll"
            theme="light"
            defaultSelectedKeys={["1"]}
            defaultOpenKeys={["sub1"]}
            mode="inline"
            items={menuItems}
          />
          <MenuDown />
        </Sider>
      </div>
    );
  };
}

export default Chainlayout;
