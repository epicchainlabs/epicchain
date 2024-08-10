/* eslint-disable */
import React from "react";
import "antd/dist/antd.min.css";
import "../../static/css/menu.css";
import "../../static/css/wallet.css";
import { Link } from "react-router-dom";
import { Layout, Menu } from "antd";
import MenuDown from "../Common/menudown";
import { HomeOutlined, WalletOutlined } from "@ant-design/icons";
import { withTranslation } from "react-i18next";

const { Sider } = Layout;

@withTranslation()
class Walletlayout extends React.Component {
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
            <WalletOutlined />
            <span>{t("sideBar.wallet")}</span>
          </span>),
        key: "sub1",
        children: [
          { label: (<Link to="/wallet/walletlist">{t("sideBar.accounts")}</Link>), key: 1 },
          { label: (<Link to="/wallet/transaction">{t("sideBar.transaction Records")}</Link>), key: 2 },
          { label: (<Link to="/wallet/transfer">{t("sideBar.transfer")}</Link>), key: 3 }
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

export default Walletlayout;
