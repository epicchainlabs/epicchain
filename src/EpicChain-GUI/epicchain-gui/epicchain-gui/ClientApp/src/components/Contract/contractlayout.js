/* eslint-disable */
import React from "react";
import { Layout, Menu, Input } from "antd";
import { Link } from "react-router-dom";
import MenuDown from "../Common/menudown";
import { HomeOutlined, FileSyncOutlined, ArrowRightOutlined, SearchOutlined } from "@ant-design/icons";
import { withTranslation } from "react-i18next";

const { Sider } = Layout;

@withTranslation()
class Contractlayout extends React.Component {
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
            <FileSyncOutlined />
            <span>{t("sideBar.contract")}</span>
          </span>),
        key: "sub1",
        children: [
          { label: (<Link to="/contract">{t("sideBar.search contract")}</Link>), key: 1 },
          { label: (<Link to="/contract/deploy">{t("sideBar.deploy contract")}</Link>), key: 2 },
          { label: (<Link to="/contract/invoke">{t("sideBar.invoke contract")}</Link>), key: 3 },
          { label: (<Link to="/contract/upgrade">{t("sideBar.upgrade contract")}</Link>), key: 4 }
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

export default Contractlayout;
