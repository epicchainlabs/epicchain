/* eslint-disable */
import React from "react";
import { Layout, Menu } from "antd";
import { Link } from "react-router-dom";
import MenuDown from "../Common/menudown";
import { HomeOutlined, DisconnectOutlined } from "@ant-design/icons";
import { withTranslation } from "react-i18next";

const { Sider } = Layout;

@withTranslation()
class Advancedlayout extends React.Component {
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
            <DisconnectOutlined />
            <span>{t("home.advanced")}</span>
          </span>),
        key: "sub1",
        children: [
          { label: (<Link to="/advanced">{t("advanced.tools")}</Link>), key: 1 },
          { label: (<Link to="/advanced/candidate">{t("advanced.candidate")}</Link>), key: 2 },
          { label: (<Link to="/advanced/vote">{t("advanced.vote")}</Link>), key: 3 },
          { label: (<Link to="/advanced/signature">{t("advanced.signature")}</Link>), key: 4 },
          { label: (<Link to="/advanced/committee">{t("advanced.committee")}</Link>), key: 5 }
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

export default Advancedlayout;
