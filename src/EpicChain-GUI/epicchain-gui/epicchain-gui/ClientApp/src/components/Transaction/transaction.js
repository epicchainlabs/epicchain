/* eslint-disable */
//just test replace wallet//
import React from 'react';
import { Link } from 'react-router-dom';
import { Layout, Row, Col, List, Button, PageHeader, message } from 'antd';
import { withTranslation } from "react-i18next";
import { SwapRightOutlined } from '@ant-design/icons';
import { postAsync } from "../../core/request";
import withRouter from '../../core/withRouter';

@withTranslation()
@withRouter
class Transaction extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      local: "",
      allpage: 0,
      page: 1,
      limit: 50,
      params: {},
      data: [],
      translist: [],
      loading: true,
      iswa: false,
      show: false
    };
  }
  componentDidMount() {
    this.setState({
      local: "/chain/transaction:"
    })
    this.selTrans()
  }
  selTrans = () => {
    let _hash = location.pathname.split(":").pop()
    let page = this.props.page ? this.props.page : "all";
    var _params = this.madeParams();
    if (page === "all") {
      this.allset(_params);
    } else if (page === "blockdetail") {
      _params.blockHeight = Number(_hash);
      this.setState({ params: _params })
      this.allset(_params);
    } else if (page === "reblockdetail") {
      _params.blockHeight = Number(_hash);
      this.setState({ params: _params })
      this.allset(_params);
    }
    // else if (page === "addressdetail") {
    //   _params.address = Number(_hash);
    //   this.setState({
    //     params: _params,
    //     local: "/wallet/transaction:"
    //   })
    //   this.nepset(_params);
    // } 
    else if (page === "assetdetail") {
      _params.asset = _hash;
      this.setState({ params: _params })
      this.nepset(_params);
    } else if (page === "wallettrans") {
      this.setState({ local: "/wallet/transaction:" });
      this.walletset(_params);
    } else if (page === "walletdetail") {
      this.setState({ local: "/wallet/transaction:" });
      _params.address = _hash;
      this.walletset(_params);
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
    this.getAlltrans(params, res => {
      this.setState({
        loading: false,
        data: res.result.list,
        translist: res.result.list,
        page: this.state.page + 1,
        allpage: Math.ceil(res.result.totalCount / this.state.limit)
      });
    })
  }
  nepset = params => {
    this.getNeptrans(params, res => {
      this.setState({
        loading: false,
        data: res.result.list,
        translist: res.result.list,
        page: this.state.page + 1,
        iswa: false,
        isnpe: true,
        allpage: Math.ceil(res.result.totalCount / this.state.limit)
      });
    })
  }
  walletset = params => {
    this.getMytrans(params, res => {
      this.setState({
        loading: false,
        data: res.result.list,
        translist: res.result.list,
        page: this.state.page + 1,
        iswa: true,
        isnpe: false,
        allpage: Math.ceil(res.result.totalCount / this.state.limit)
      });
    })
  }
  getMytrans = async (params, callback) => {
    let response = await postAsync("GetMyTransactions", params);
    if (response.msgType === -1) {
      message.error("查询失败");
      return;
    } else {
      callback(response);
    }
  }
  getAlltrans = async (params, callback) => {
    let response = await postAsync("QueryTransactions", params);
    if (response.msgType === -1) {
      console.log(response)
      message.error("查询失败");
      return;
    } else {
      callback(response);
    }
  };
  getNeptrans = async (params, callback) => {
    let response = await postAsync("QueryNep5Transactions", params);
    if (response.msgType === -1) {
      console.log(response)
      message.error("查询失败");
      return;
    } else {
      callback(response);
    }
  };
  loadMore = () => {
    this.setState({
      loading: true,
    });
    var _flag = this.madeParams();
    var _params = Object.assign(this.state.params, _flag);
    this.getAlltrans(_params, res => {
      const data = this.state.data.concat(res.result.list);
      const _page = this.state.page + 1;
      this.setState(
        {
          data: data,
          translist: data,
          loading: false,
          page: _page
        },
        () => {
          window.dispatchEvent(new Event('resize'));
        },
      );
    });
  }
  loadMyMore = () => {
    this.setState({
      loading: true,
    });
    var _params = this.madeParams();
    this.getMytrans(_params, res => {
      const data = this.state.data.concat(res.result.list);
      const _page = this.state.page + 1;
      this.setState(
        {
          data: data,
          translist: data,
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
  loadNepMore = () => {
    this.setState({
      loading: true,
    });
    var _flag = this.madeParams();
    var _params = Object.assign(this.state.params, _flag);
    this.getNeptrans(_params, res => {
      const data = this.state.data.concat(res.result.list);
      const _page = this.state.page + 1;
      this.setState(
        {
          data: data,
          translist: data,
          loading: false,
          page: _page
        },
        () => {
          window.dispatchEvent(new Event('resize'));
        },
      );
    });
  }
  visi = () => {
    this.setState({
      show: !this.state.show,
    });
  }
  show = (e) => {
    return () => {
      console.log(this.state.show)
    }
  }
  render() {
    const { t } = this.props;
    const { location } = this.props.router;
    const { translist, local, loading, iswa, isnpe, page, allpage } = this.state;
    const loadMore = !loading && page <= allpage ? (
      <div className="text-c mb3">
        {iswa ? (<Button type="primary" onClick={this.loadMyMore}>{t('common.load more')}</Button>)
          : isnpe ? (<Button type="primary" onClick={this.loadNepMore}>{t('common.load more')}</Button>) :
            (<Button type="primary" onClick={this.loadMore}>{t('common.load more')}</Button>)}
      </div>
    ) : null;
    const path = location.pathname || null;
    return (
      <div>
        <List
          header={<div><span>{t("blockchain.transaction info")}</span><span className="float-r">{t("common.time")}</span></div>}
          footer={<span></span>}
          itemLayout="horizontal"
          loading={loading}
          loadMore={loadMore}
          dataSource={translist}
          className="font-s"
          renderItem={item => (
            <List.Item>
              {/* <List.Item.Meta
              title={<span className="succes-light">{t('blockchain.transaction.confirmed')}</span>}
              /> */}
              <div className="trans-detail">
                <p>
                  <Link className="w530 ellipsis hash code"
                    to={{
                      pathname: local + item.txId,
                      state: { from: path }
                    }}
                    title={t("show detail")}
                    state={{ from: path }}
                  > {item.txId}</Link>
                  <span className="float-r">{item.blockTime}</span>
                </p>
                {item.transfers?.length > 0 ?
                  <div>
                    <span className="w200 ellipsis">{item.transfers[0].fromAddress ? item.transfers[0].fromAddress : "--"}</span>
                    <SwapRightOutlined />
                    <span className="w200 ellipsis" >{item.transfers[0].toAddress ? item.transfers[0].toAddress : "--"}</span>
                    <span className="float-r"><span className="trans-amount">{item.transfers[0].amount}</span>{item.transfers[0].symbol}</span>
                  </div>
                  : null}
              </div>
            </List.Item>
          )
          }
        />
        {/* <Searcharea show={this.show()} />
        <div className="pv1"></div> */}
      </div >
    );
  }
}

export default Transaction;