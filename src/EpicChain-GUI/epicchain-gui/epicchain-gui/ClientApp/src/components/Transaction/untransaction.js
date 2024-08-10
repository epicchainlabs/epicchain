/* eslint-disable */
//just test replace wallet//
import React from 'react';
import { Link } from 'react-router-dom';
import { Layout, Row, Col, PageHeader, List, Button, message } from 'antd';
import { withTranslation } from "react-i18next";
import { post } from "../../core/request";
import withRouter from '../../core/withRouter';

@withTranslation()
@withRouter
class Untransaction extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      local: "",
      allpage: 0,
      page: 1,
      limit: 3,
      params: {},
      data: [],
      untranslist: [],
      loading: true,
      iswa: false,
    };
  }
  componentDidMount() {
    this.setState({
      local: "/chain/untransaction:"
    })
    this.selTrans()
  }
  selTrans = () => {
    let page = this.props.page ? this.props.page : "all";
    var _params = this.madeParams();
    if (page === "all") {
      this.allset(_params);
    } else if (page === "wallet") {
      this.walletset(_params);
      this.setState({
        local: "/wallet/untransaction:"
      })
    } else {
      this.allset(_params);
    }
  }
  madeParams = () => {
    return {
      "pageIndex": this.state.page,
      "limit": this.state.limit
    };
  }
  allset = params => {
    this.getAlluntrans(params, res => {
      this.setState({
        loading: false,
        data: res.result.list,
        untranslist: res.result.list,
        page: this.state.page + 1,
        iswa: false,
        allpage: Math.ceil(res.result.totalCount / this.state.limit)
      });
    })
  }
  walletset = params => {
    this.getMyuntrans(params, res => {
      this.setState({
        loading: false,
        data: res.result.list,
        untranslist: res.result.list,
        page: this.state.page + 1,
        iswa: true,
        allpage: Math.ceil(res.result.totalCount / this.state.limit)
      });
    })
  }
  getMyuntrans = (params, callback) => {
    post("GetMyUnconfirmedTransactions", params).then(res => {
      var _data = res.data;
      if (_data.msgType === -1) {
        message.error("查询失败");
        return;
      } else {
        callback(_data);
      }
    });
  };
  getAlluntrans = (params, callback) => {
    post("GetUnconfirmTransactions", params).then(res => {
      var _data = res.data;
      if (_data.msgType === -1) {
        message.error("查询失败");
        return;
      } else {
        callback(_data);
      }
    });
  };
  loadUnMore = () => {
    this.setState({
      loading: true,
    });
    var _params = this.madeParams();
    this.getAlluntrans(_params, res => {
      const data = this.state.data.concat(res.result.list);
      const _page = this.state.page + 1;
      this.setState(
        {
          data: data,
          untranslist: data,
          loading: false,
          page: _page
        },
        () => {
          window.dispatchEvent(new Event('resize'));
          console.log(this.state);
        },
      );
    });
  }
  loadMyUnMore = () => {
    this.setState({
      loading: true,
    });
    var _params = this.madeParams();
    this.getMyuntrans(_params, res => {
      const data = this.state.data.concat(res.result.list);
      const _page = this.state.page + 1;
      this.setState(
        {
          data: data,
          untranslist: data,
          loading: false,
          page: _page
        },
        () => {
          window.dispatchEvent(new Event('resize'));
          console.log(this.state);
        },
      );
    });
  }
  render() {
    const { t } = this.props;
    const { location } = this.props.router;
    const { untranslist, loading, iswa, page, allpage, local } = this.state;
    const loadUnMore = !loading && page <= allpage ? (
      <div className="text-c mb3">
        {iswa ? (<Button type="primary" onClick={this.loadMyUnMore}>{t('common.load more')}</Button>)
          : (<Button type="primary" onClick={this.loadUnMore}>{t('common.load more')}</Button>)}
      </div>
    ) : null;
    const path = location.pathname || null;
    return (
      <div>
        <List
          header={<div><span>{t("blockchain.transaction info")}</span></div>}
          footer={<span></span>}
          itemLayout="horizontal"
          loading={loading}
          loadMore={loadUnMore}
          dataSource={untranslist}
          className="font-s"
          renderItem={item => (
            <List.Item>
              {/* <List.Item.Meta
              title={<span className="fail-light">{t('blockchain.transaction.unconfirmed')}</span>}
              /> */}
              <div className="trans-detail">
                <p>
                  <Link className="w530 ellipsis hash"
                    to={{
                      pathname: local + item.txId,
                      state: { from: path }
                    }}
                    title={t("show detail")}
                    state={{ from: path }}
                  >{item.txId}</Link>
                  <span className="float-r">{item.blockTime}</span>
                </p>
              </div>
            </List.Item>
          )}
        />
      </div>
    );
  }
}

export default Untransaction;