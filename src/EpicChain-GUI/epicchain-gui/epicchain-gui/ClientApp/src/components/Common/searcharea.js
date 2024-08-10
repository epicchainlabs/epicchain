/* eslint-disable */
import React, { useState, createRef } from "react";
import "antd/dist/antd.min.css";
import { useHistory } from "react-router-dom";
import { Input, message } from "antd";
import Topath from "../Common/topath";
import { post, postAsync } from "../../core/request";
import { ArrowRightOutlined, SearchOutlined } from "@ant-design/icons";
import { withTranslation, useTranslation } from "react-i18next";

@withTranslation()
class Searcharea extends React.Component {
  constructor(props) {
    super(props);
    this.searchInput = createRef();
    this.state = {
      size: "default",
      path: "",
      disabled: false,
      cname: "search-content",
    };
  }
  addClass = (e) => {
    this.stopPropagation(e);
    this.setState({
      cname: "search-content height-sea show-child",
      disabled: true,
    });
    document.addEventListener("click", this.removeClass);
  };
  removeClass = () => {
    if (this.state.disabled) {
      this.setState({
        cname: "search-content height-sea",
        disabled: false,
      });
    }
    document.removeEventListener("click", this.removeClass);
    setTimeout(
      () =>
        this.setState({
          cname: "search-content",
          disabled: false,
        }),
      500
    );
  };
  stopPropagation(e) {
    e.nativeEvent.stopImmediatePropagation();
  }
  searchContract = async () => {
    const { t } = this.props;
    let _hash = this.searchInput.current.input.value.trim();
    if (!_hash) {
      message.info(t("search.check again"));
      return;
    }
    let response = await postAsync("GetContract", {
      contractHash: _hash,
    });
    if (response.msgType === -1) {
      message.info(t("search.hash unexist"));
      return;
    }
    this.setState({ topath: "/contract/detail:" + _hash });
  };
  render() {
    const { t } = this.props;
    return (
      <div className="search-area">
        <Topath topath={this.state.topath}></Topath>
        <div className="search-btn">
          <SearchOutlined className="inset-btn" onClick={this.addClass} />
        </div>
        <div className={this.state.cname}>
          <div
            className="search-detail"
            onClick={this.stopPropagation}
          >
            <Input
              placeholder={t("search.hash-hint")}
              onPressEnter={this.searchContract}
              ref={this.searchInput}
              suffix={<ArrowRightOutlined onClick={this.searchContract} />}
            />
          </div>
        </div>
      </div>
    );
  };
}

const Searchtttt = () => {
  // console.log(this.props)
  const { t } = useTranslation();
  const [hash, searchHash] = useState(false);
  let history = useHistory();
  const aaa = (value) => {
    console.log(value);
  };
  const sessss = () => {
    let txid =
      "0xc5c29246d1f1e05efffe541e6951ff60f6c5e03e2f246c1754c182604fad1105";
    console.log(txid);
    // let params = { "txId":txid };
    // post("GetTransaction",params).then(function (res) {
    //   var _data = res.data;
    //   if(_data.msgType === -1){
    //     message.info(t('search.hash unexist'));
    //     return;
    //   }else if(_data.msgType === 3){
    //     message.info("hashcunzai");
    //     history.push("/wallet/transaction:"+txid);
    //   }
    // })
    // .catch(function (error) {
    //   console.log(error);
    //   console.log("error");
    // });
  };
  return (
    <div>
      <SearchOutlined className="h2" onClick={aaa} />
      <div className="search-div">
        <Input
          placeholder={'t("search.hash-hint")'}
          onPressEnter={(value) => {
            console.log(value);
          }}
          suffix={<ArrowRightOutlined onClick={sessss} />}
        />
      </div>
    </div>
  );
};

export { Searchtttt };
export default Searcharea;
