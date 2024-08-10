/* eslint-disable */
import React, { createRef } from "react";
import "antd/dist/antd.min.css";
import { Input, message } from "antd";
import Topath from "../Common/topath";
import { ArrowRightOutlined, SearchOutlined } from "@ant-design/icons";
import { withTranslation } from "react-i18next";
import { postAsync } from "../../core/request";

@withTranslation()
class Chainsearch extends React.Component {
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
  searchChain = async () => {
    const { t } = this.props;
    let input = this.searchInput.current.input.value.trim();
    let blockHeight = 0;
    let blockHash = null;
    if (input.length == 66) {
      blockHash = input
    } else {
      blockHeight = Number(input);
    }
    if (!input) {
      message.info(t("search.check again"));
      return;
    }
    let response = !blockHash ? (await postAsync("GetBlock", { index: blockHeight })) : (await postAsync("GetBlockByHash", { hash: blockHash }));
    if (response.msgType === -1) {
      message.info(t("blockchain.search input-invalid"));
      return;
    }
    this.setState({ topath: "/chain/detail:" + response.result.blockHeight });
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
              placeholder={t("search.chain-hint")}
              onPressEnter={this.searchChain}
              ref={this.searchInput}
              suffix={<ArrowRightOutlined onClick={this.searchChain} />}
            />
          </div>
        </div>
      </div>
    );
  };
}

export default Chainsearch;
