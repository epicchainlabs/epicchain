/* eslint-disable */
import React, { createRef } from "react";
import "antd/dist/antd.min.css";
import { Input, message } from "antd";
import Topath from "../Common/topath";
import { ArrowRightOutlined, SearchOutlined } from "@ant-design/icons";
import { withTranslation } from "react-i18next";
import { postAsync } from "../../core/request";

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

    let response = await postAsync("GetTransaction", {
      txId: _hash,
    });
    if (response.msgType === -1) {
      message.info(t("search.check again"));
      return;
    }
    if (response.msgType === 3) {
      this.setState({ topath: "transaction:" + _hash });
    }
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

export default Searcharea;
