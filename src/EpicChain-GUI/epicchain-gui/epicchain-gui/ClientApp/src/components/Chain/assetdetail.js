/* eslint-disable */
//just test replace wallet//
import React, { useState, useEffect } from "react";
import { Layout, Row, Col, message, List, PageHeader } from "antd";
import Transaction from "../Transaction/transaction";
import Sync from "../sync";
import { withTranslation, useTranslation } from "react-i18next";
import { postAsync } from "../../core/request";


export default function AssetDetail() {
  const { Content } = Layout;
  const [assetdetail, setAssetDetail] = useState({});
  const { t } = useTranslation();

  useEffect(() => {
    let params = {
      asset: location.pathname.split(":").pop(),
    };
    postAsync("GetAsset", params).then((data) => {
      if (data.msgType < 0) {
        message.error(t("alert msg.no find"));
        return;
      }
      setAssetDetail(data.result);
    }).catch(function (error) {
      console.log(error);
    });
  }, []);
  return (
    <Layout className="gui-container">
      <Sync />
      <Content className="mt3">
        <Row gutter={[30, 0]} type="flex">
          <Col span={24} className="bg-white pv4">
            <PageHeader title={t("blockchain.asset detail")}></PageHeader>
            <div className="info-detail pv3">
              <div className="hash-title pa3 mt5 mb4">
                <span>Hash: &nbsp;&nbsp;&nbsp;</span>
                {assetdetail.asset}
              </div>
              {assetdetail.asset ? (
                <Row>
                  <Col span={12}>
                    <ul className="detail-ul">
                      <li>
                        <span className="hint">{t("blockchain.name")}:</span>
                        {assetdetail.name}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.total")}：
                        </span>
                        {assetdetail.totalSupply
                          ? assetdetail.totalSupply
                          : "--"}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.publish time")}：
                        </span>
                        {assetdetail.createTime.substr(0, 10)}
                      </li>
                    </ul>
                  </Col>
                  <Col span={12}>
                    <ul className="detail-ul">
                      <li>
                        <span className="hint">
                          {t("blockchain.abbreviation")}：
                        </span>
                        {assetdetail.symbol}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.precision")}：
                        </span>
                        {assetdetail.decimals ? assetdetail.decimals : "0"}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.transaction count")}：
                        </span>
                        {assetdetail.transactionCount
                          ? assetdetail.transactionCount
                          : "--"}
                      </li>
                    </ul>
                  </Col>
                </Row>
              ) : null}
            </div>
          </Col>
        </Row>

        <Row
          gutter={[30, 0]}
          className="mt2 mb1"
          type="flex"
          style={{ minHeight: "120px" }} >
          <Col span={24} className="bg-white pv4">
            <PageHeader title={t("blockchain.transactions")}></PageHeader>
            <Transaction
              content={t("blockchain.transactions")}
              page="assetdetail"
            />
          </Col>
        </Row>
      </Content>
    </Layout>
  );
}

