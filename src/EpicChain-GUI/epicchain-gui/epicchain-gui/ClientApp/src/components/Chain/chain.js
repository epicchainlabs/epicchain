/* eslint-disable */
//just test replace wallet//
import React from "react";
import { Link } from "react-router-dom";
import {
  Layout,
  Row,
  Col,
  List,
  Typography,
  message,
  Button,
  PageHeader,
} from "antd";
import Chainsearch from "./chainSearch";
import Sync from "../sync";
import { withTranslation } from "react-i18next";
import "../../static/css/contract.css";
import { postAsync } from "../../core/request";

const { Content } = Layout;

const count = 3;
@withTranslation()
class Chain extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      loading: false,
      initLoading: true,
      data: [],
      blocklist: [],
      visible: false,
      show: false,
    };
  }
  componentDidMount() {
    this.getBlock((res) => {
      this.setState(
        {
          initLoading: false,
          data: res.result,
          blocklist: res.result,
          lastblock: res.result[res.result.length - 1].blockHeight - 1,
        },
        () => { }
      );
    });
  }
  getBlock = async (callback) => {
    console.log(this.state.lastblock);
    let _params = this.state.lastblock
      ? {
        limit: 50,
        height: this.state.lastblock,
      }
      : {
        limit: 50,
      };
    let response = await postAsync("GetLastBlocks", _params);
    if (response.msgType < 0) {
      message.error("Query fail!");
      return;
    }
    callback(response)
  };
  loadMore = () => {
    this.setState({
      loading: true,
      blocklist: this.state.data.concat(
        [...new Array(count)].map(() => ({ loading: true, name: {} }))
      ),
    });
    this.getBlock((res) => {
      const data = this.state.data.concat(res.result);
      this.setState(
        {
          data,
          blocklist: data,
          loading: false,
          lastblock: data[data.length - 1].blockHeight - 1,
        },
        () => {
          window.dispatchEvent(new Event("resize"));
        }
      );
    });
  };
  show = (e) => {
    return () => {
      console.log(this.state.show);
    };
  };
  render() {
    const { t } = this.props;
    const { initLoading, loading, blocklist } = this.state;
    const loadMore =
      !initLoading && !loading ? (
        <div className="text-c mb3">
          <Button type="primary" onClick={this.loadMore}>
            {t("common.load more")}
          </Button>
        </div>
      ) : null;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row gutter={[30, 0]} style={{ minHeight: "calc( 100vh - 120px )" }}>
            <Col span={24} className="bg-white pv4">
              <PageHeader title={t("blockchain.blocks")}></PageHeader>
              <List
                header={
                  <div>
                    <span>{t("blockchain.block info")}</span>
                    <span className="float-r ml4">
                      <span className="wa-amount">
                        {t("blockchain.transaction count")}
                      </span>
                    </span>
                    <span className="float-r">
                      {t("blockchain.block time")}
                    </span>
                  </div>
                }
                footer={<span></span>}
                itemLayout="horizontal"
                loading={initLoading}
                loadMore={loadMore}
                dataSource={blocklist}
                className="font-s"
                renderItem={(item) => (
                  <List.Item>
                    <List.Item.Meta
                      title={
                        <Link
                          to={"/chain/detail:" + item.blockHeight}
                          title={t("show detail")}
                        >
                          {item.blockHeight}
                        </Link>
                      }
                      description={
                        <div className="font-s">{item.blockHash}</div>
                      }
                    />
                    <Typography>{item.blockTime}</Typography>
                    <Typography className="upcase ml4">
                      <span className="wa-amount">{item.transactionCount}</span>
                    </Typography>
                  </List.Item>
                )}
              />
            </Col>
            <Chainsearch show={this.show()} />
          </Row>
          <div className="pv2"></div>
        </Content>
      </Layout>
    );
  }
}

export default Chain;
