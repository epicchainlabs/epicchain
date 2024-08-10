import React from "react";
import { Link } from "react-router-dom";
import { Row, Col, Divider, Select, Radio } from "antd";
import { Empty } from "antd";
import { useTranslation, withTranslation } from "react-i18next";
import "../../static/css/trans.css";
import { useState } from "react";

const { Option } = Select;

const Hashdetail = ({ hashdetail }) => {
  const { t } = useTranslation();
  return (
    <Row>
      <div className="hash-title pa3">
        <span>Hash: &nbsp;&nbsp;&nbsp;</span>
        {hashdetail.txId}
      </div>
      <Col span={12}>
        <ul className="detail-ul">
          <li>
            <span className="hint">{t("blockchain.block")}：</span>
            <Link to={"/chain/detail:" + hashdetail.blockHeight}>
              {hashdetail.blockHeight ? hashdetail.blockHeight : "--"}
            </Link>
          </li>
          <li>
            <span className="hint">{t("blockchain.timestamp")}：</span>
            {hashdetail.timestamp ? hashdetail.timestamp : "--"}
          </li>
          <li>
            <span className="hint">{t("blockchain.network fee")}：</span>
            {hashdetail.networkFee ? hashdetail.networkFee : "--"} GAS
          </li>
          <li>
            <span className="hint">{t("blockchain.confirmations")}：</span>
            {hashdetail.confirmations ? hashdetail.confirmations : "--"}
          </li>
        </ul>
      </Col>
      <Col span={12}>
        <ul className="detail-ul">
          <li>
            <span className="hint">{t("common.size")}：</span>
            {hashdetail.size ? hashdetail.size : "--"} {t("common.bytes")}
          </li>
          <li>
            <span className="hint">{t("common.time")}：</span>
            {hashdetail.blockTime ? hashdetail.blockTime : "--"}
          </li>
          <li>
            <span className="hint">{t("blockchain.system fee")}：</span>
            {hashdetail.systemFee ? hashdetail.systemFee : "--"} GAS
          </li>
          <li>
            <span className="hint">{t("blockchain.nounce")}：</span>
            {hashdetail.nonce ? hashdetail.nonce : "--"}
          </li>
        </ul>
      </Col>
      <div className="hash-title pv3"></div>
      <Divider></Divider>
    </Row>
  );
};

const Translist = ({ transfers }) => {
  const { t } = useTranslation();
  transfers = transfers ? transfers : [];
  return (
    <div>
      {transfers.length > 0 ? (
        <Row>
          <Col span={24}>
            <div className="hash-title pa3 mt4 mb3">
              {t("blockchain.transaction.transfers")}
            </div>
            {transfers.map((item, index) => {
              return (
                <ul className="detail-ul border-under" key={index}>
                  <li>
                    <span className="gray">
                      {t("blockchain.transaction.from")}
                    </span>
                    <span className="detail-add">
                      {item.fromAddress ? item.fromAddress : "--"}
                    </span>
                    <span className="gray">
                      {t("blockchain.transaction.to")}
                    </span>
                    <span className="detail-add">
                      {item.toAddress ? item.toAddress : "--"}
                    </span>
                  </li>
                  <li>
                    <span className="gray">
                      {t("blockchain.transaction.amount")}
                    </span>
                    <span className="detail-amount">
                      {item.amount} {item.symbol}
                    </span>
                  </li>
                </ul>
              );
            })}
          </Col>
        </Row>
      ) : null}
    </div>
  );
};

const Attrlist = ({ attributes }) => {
  const { t } = useTranslation();
  attributes = attributes ? attributes : [];
  return (
    <div>
      {attributes.length > 0 ? (
        <Row>
          <Col span={24}>
            <div className="hash-title pa3 mt5 mb4">
              {t("blockchain.transaction.attributes")}
            </div>
            <ul className="detail-ul">
              {attributes.map((item, index) => {
                return (
                  <li key={index}>
                    <p className="trans-table">
                      <span>
                        <span className="trans-type">
                          {item.type ? item.type : "--"}
                        </span>
                      </span>
                      <span>{item.data ? item.data : "--"}</span>
                    </p>
                  </li>
                );
              })}
            </ul>
          </Col>
          <Divider></Divider>
        </Row>
      ) : null}
    </div>
  );
};

const Witlist = ({ witnesses }) => {
  const { t } = useTranslation();
  const [opClass, changeOP] = useState("showhex");
  witnesses = witnesses ? witnesses : [];
  if (witnesses.length <= 0) return null;
  return (
    <Row>
      <Col span={24}>
        <div className="hash-title pa3 mt4 mb4">
          {t("blockchain.witness")}
          <Radio.Group
            onChange={(e) => changeOP(e.target.value)}
            defaultValue="showhex"
          >
            <Radio className="font-s ml1" value="showhex">
              Hex
            </Radio>
            <Radio className="font-s" value="showopcode">
              Opcode
            </Radio>
          </Radio.Group>
        </div>

        {witnesses.map((item, index) => {
          return (
            <div className={"detail-ul border-under " + opClass} key={index}>
              <ul className="hex">
                <li>
                  <p>{t("blockchain.transaction.invocation script")}</p>
                  <p className="trans-table">
                    <span>{item.invocationScript}</span>
                  </p>
                </li>
                <li>
                  <p>{t("blockchain.transaction.verification script")}</p>
                  <p className="trans-table">
                    <span>{item.verificationScript}</span>
                  </p>
                </li>
              </ul>

              <ul className="opcode">
                <li>
                  <p>{t("blockchain.transaction.invocation script")}</p>
                </li>
                {item.invocationOpCode.map((i, index) => {
                  return (
                    <li key={index}>
                      <p className="trans-table">
                        <span className="trans-type gray">{i.opCodeName}</span>
                        <span>{i.opDataPossibleString}</span>
                      </p>
                    </li>
                  );
                })}
                <li>
                  <p>{t("blockchain.transaction.verification script")}</p>
                </li>
                {item.verificationOpCode.map((i, index) => {
                  return (
                    <li key={index}>
                      <p className="trans-table">
                        <span className="trans-type gray">{i.opCodeName}</span>
                        <span>{i.opDataPossibleString}</span>
                      </p>
                    </li>
                  );
                })}
              </ul>
            </div>
          );
        })}
      </Col>
    </Row>
  );
};

const Scriptlist = ({ script, scriptcode }) => {
  const { t } = useTranslation();
  const [opClass, changeOP] = useState("showhex");
  script = script ? script : "";
  if (script === "") return null;
  return (
    <Row>
      <Col span={24}>
        <div className="hash-title pa3 mt4 mb4">
          {t("blockchain.transaction.script")}
          <Radio.Group
            onChange={(e) => changeOP(e.target.value)}
            defaultValue="showhex"
          >
            <Radio className="font-s ml1" value="showhex">
              Hex
            </Radio>
            <Radio className="font-s" value="showopcode">
              Opcode
            </Radio>
          </Radio.Group>
        </div>

        <div className={"detail-ul border-under " + opClass}>
          <ul className="hex">
            <li>{script}</li>
          </ul>

          <ul className="opcode">
            {scriptcode.map((i, index) => {
              return (
                <li key={index}>
                  <p className="trans-table">
                    <span className="trans-type gray">{i.opCodeName}</span>
                    <span>{i.opDataPossibleString}</span>
                  </p>
                </li>
              );
            })}
          </ul>
        </div>
      </Col>
    </Row>
  );
};

@withTranslation()
class Notifies extends React.Component {
  constructor(props) {
    super(props);
    this.state = {};
  }
  render() {
    const { t } = this.props;
    const { notifies } = this.props;
    return (
      <div className="info-detail">
        {notifies.length === 0 ? (
          <Empty image={Empty.PRESENTED_IMAGE_SIMPLE} />
        ) : null}
        <ul className="trans-ul">
          {notifies.map((item) => {
            let _data = item.state ? item.state.value : [];
            var html = [];
            html.push(
              <li className="trans-title pa3" key="title">
                <span>ScriptHash: &nbsp;&nbsp;&nbsp;</span>
                {item.contract}
              </li>
            );
            html.push(
              <li className="pa3 bold t-dark" key="event">
                <span className="trans-type">Event</span>
                {item.eventName}
              </li>
            );
            for (var i = 0; i < _data.length; i++) {
              html.push(
                <li className="pa3" key={i}>
                  <span className="trans-type">{_data[i].type}</span>
                  {_data[i].value
                    ? JSON.stringify(_data[i].value).replace(/"/g, " ")
                    : "--"}
                </li>
              );
            }

            html.push(<Divider key="divider"></Divider>);
            return html;
          })}
        </ul>
      </div>
    );
  };
}

export { Hashdetail, Attrlist, Translist, Witlist, Scriptlist };
export default Notifies;
