/* eslint-disable */
//just test replace wallet//
import React, { useState, useEffect } from "react";
import { Link, useLocation } from "react-router-dom";
import { Layout, Row, Col, message, PageHeader, List } from "antd";
import { SwapRightOutlined } from "@ant-design/icons";
import Sync from "../sync";
import { postAsync } from "../../core/request";
import { withTranslation, useTranslation } from "react-i18next";


export default function BlockDetail() {
  const { Content } = Layout;
  const { t } = useTranslation();
  const location = useLocation();
  const path = location.pathname || null;

  const [height, setHeight] = useState(0);
  const [blockdetail, setBlockDetail] = useState({});
  const [translist, setTransList] = useState([]);
  let identity = location.pathname.split(":").pop();
  let blockHeight = Number(identity);
  useEffect(() => {
    postAsync("GetBlock", { index: blockHeight })
      .then(function (data) {
        if (data.msgType < 0) {
          message.info(t("blockchain.height unexist"));
          return;
        }
        setBlockDetail(data.result);
      });

    postAsync("QueryTransactions", { blockHeight: blockHeight, limit: 500, })
      .then((msg) => {
        if (msg.msgType < 0) {
          message.info(t("blockchain.height unexist"));
          return;
        }
        setTransList(msg.result.list);
      });
  }, [height]);
  return (
    <Layout className="gui-container">
      <Sync />
      <Content className="mt3">
        <Row gutter={[30, 0]} type="flex">
          <Col span={24} className="bg-white pv4">
            <PageHeader title={t("blockchain.block info")}></PageHeader>
            <div className="info-detail pv3">
              <div className="hash-title pa3 mt5 mb4">
                <span>Hash: &nbsp;&nbsp;&nbsp;</span>
                {blockdetail.blockHash}
              </div>
              {blockdetail.blockHash ? (
                <Row>
                  <Col span={12}>
                    <ul className="detail-ul">
                      <li>
                        <span className="hint">
                          {t("blockchain.height")}:
                        </span>
                        {blockdetail.blockHeight}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.timestamp")}：
                        </span>
                        {blockdetail.blockTime}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.network fee")}：
                        </span>
                        {blockdetail.networkFee
                          ? blockdetail.networkFee
                          : "--"}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.confirmations")}：
                        </span>
                        {blockdetail.confirmations}
                      </li>
                      {blockdetail.blockHeight !== 0 ? (
                        <li>
                          <span className="hint">
                            {t("blockchain.prev block")}：
                          </span>
                          <Link
                            to={
                              "/chain/detail:" + (blockdetail.blockHeight - 1)
                            }
                            onClick={() => setHeight(blockdetail.blockHeight - 1)}
                          >
                            {blockdetail.blockHeight - 1}
                          </Link>
                        </li>
                      ) : (
                        <li>
                          <span className="hint">
                            {t("blockchain.prev block")}：
                          </span>
                          --
                        </li>
                      )}
                    </ul>
                  </Col>
                  <Col span={12}>
                    <ul className="detail-ul">
                      <li>
                        <span className="hint">{t("common.size")}：</span>
                        {blockdetail.size} {t("common.bytes")}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.primary index")}：
                        </span>
                        {blockdetail.primaryIndex}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.system fee")}：
                        </span>
                        {blockdetail.systemFee ? blockdetail.systemFee : "--"}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.witness")}：
                        </span>
                        {blockdetail.nextConsensus}
                      </li>
                      <li>
                        <span className="hint">
                          {t("blockchain.next block")}：
                        </span>
                        <Link
                          to={
                            "/chain/detail:" + (blockdetail.blockHeight + 1)
                          }
                          onClick={() => setHeight(blockdetail.blockHeight + 1)} >
                          {blockdetail.blockHeight + 1}
                        </Link>
                      </li>
                    </ul>
                  </Col>
                </Row>
              ) : null}
            </div>
          </Col>
        </Row>

        {/* <Transaction page={this.state.href} content={t("blockchain.transactions")} /> */}

        <Row
          gutter={[30, 0]}
          className="mt2 mb2"
          type="flex"
          style={{ minHeight: "120px" }} >
          <Col span={24} className="bg-white pv4">
            <PageHeader title={t("blockchain.transactions")}></PageHeader>
            <List
              header={
                <div>
                  <span className="succes-light">
                    {t("blockchain.transaction.status")}
                  </span>
                  <span>{t("blockchain.transaction info")}</span>
                  <span className="float-r">{t("common.time")}</span>
                </div>
              }
              footer={<span></span>}
              itemLayout="horizontal"
              dataSource={translist}
              className="font-s"
              renderItem={(item) => (
                <List.Item>
                  <List.Item.Meta
                    title={
                      <span className="succes-light">
                        {t("blockchain.transaction.confirmed")}
                      </span>
                    }
                  />
                  <div className="trans-detail">
                    <p>
                      <Link
                        className="w530 ellipsis hash"
                        to={{
                          pathname: "/chain/transaction:" + item.txId,
                          state: { from: path }
                        }}
                        state={{ from: path }}
                        title={t("show detail")}
                      >
                        {item.txId}
                      </Link>
                      <span className="float-r">{item.blockTime}</span>
                    </p>
                    {item.transfers?.length > 0 ? (
                      <div>
                        <span className="w200 ellipsis">
                          {item.transfers[0].fromAddress
                            ? item.transfers[0].fromAddress
                            : "--"}
                        </span>
                        <SwapRightOutlined />
                        <span className="w200 ellipsis">
                          {item.transfers[0].toAddress
                            ? item.transfers[0].toAddress
                            : "--"}
                        </span>
                        <span className="float-r">
                          <span className="trans-amount">
                            {item.transfers[0].amount}
                          </span>
                          {item.transfers[0].symbol}
                        </span>
                      </div>
                    ) : null}
                  </div>
                </List.Item>
              )}
            />
          </Col>
          <div className="pv1"></div>
        </Row>
      </Content>
    </Layout>
  );
}
