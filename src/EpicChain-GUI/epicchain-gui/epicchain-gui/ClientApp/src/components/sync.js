/* eslint-disable */
import React from "react";
import { Typography } from "antd";
import "antd/dist/antd.min.css";
import { SyncOutlined } from "@ant-design/icons";
import { observer, inject } from "mobx-react";
import { withTranslation } from "react-i18next";

const { Text } = Typography;

@withTranslation()
@inject("blockSyncStore")
@observer
class Sync extends React.Component {
  render() {
    const { t } = this.props;
    const { syncHeight, headerHeight } = this.props.blockSyncStore;
    return (
      <div className="ml3 mb0">
        {headerHeight < 0 ? (
          <Text className="t-normal bold"> - / - {t("common.connecting")}</Text>
        ) : (
          <Text className="t-normal bold">
            {" "}
            {syncHeight} / {headerHeight} {t("common.syncing")}
          </Text>
        )}
        <SyncOutlined className="ml3" type="sync" spin />
      </div>
    );
  }
}

export default Sync;
