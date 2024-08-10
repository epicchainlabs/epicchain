/* eslint-disable */
//just test replace wallet//
import React from 'react';
import _ from 'lodash';
import { Layout, Row, Col, Modal, List, Button, Typography, message, PageHeader } from 'antd';
import Sync from '../sync';
import Transaction from '../Transaction/transaction';
import '../../static/css/wallet.css';
import Topath from '../Common/topath';
import {
  CloseCircleOutlined
} from '@ant-design/icons';
import { withTranslation } from "react-i18next";
import { postAsync } from '../../core/request';
import { withAuthenticated } from '../../core/authentication';
import withRouter from '../../core/withRouter';


const { confirm } = Modal;
const { Content } = Layout;

@withRouter
@withTranslation()
@withAuthenticated
class Walletdetail extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      size: 'default',
      address: "",
      assetlist: [],
      iconLoading: false,
      gas: 0,
    };
  }
  componentDidMount() {
    this.checkAddress();
    this.getBalances();
  }
  checkAddress = () => {
    let address = location.pathname.split(":").pop();
    this.setState({ address: address })
  }
  getBalances = async () => {
    let address = location.pathname.split(":").pop();
    let data = await postAsync("GetMyBalances", {
      "address": address
    })
    if (data.msgType === -1) {
      return;
    }
    if (data.result.length > 0) {
      this.setState({
        assetlist: data.result[0]
      })
    }
  }
  // getGas = async () => {
  //   let data = await postAsync("ShowGas");
  //   if (data.msgType === -1) return;
  //   this.setState({
  //     gas: data.result.unclaimedGas
  //   })
  // }
  deleteConfirm = () => {
    let _this = this;
    let { t } = this.props;
    confirm({
      title: t("wallet.delete account warning"),
      icon: <CloseCircleOutlined />,
      cancelText: t("button.cancel"),
      okText: t("button.delete"),
      onOk() {
        _this.delAddress();
      },
      onCancel() {
        console.log('Cancel');
      },
    });
  }
  delAddress = async () => {
    const { t } = this.props;
    let data = await postAsync("DeleteAddress", [this.state.address]);
    if (data.msgType === -1) {
      return;
    }
    message.success(t("wallet.delete success"), 2)
    this.setState({ topath: "/wallet/walletlist" });
  }
  showPrivate = async () => {
    let { t } = this.props;
    let repsonse = await postAsync("ShowPrivateKey", {
      "address": this.state.address
    });
    if (repsonse.msgType === -1) {
      console.log(t('wallet.require open'));
      return;
    }
    let data = repsonse.result;
    Modal.info({
      title: t("wallet.private key warning"),
      width: 650,
      content: (
        <div className="show-pri">
          <p>{t("wallet.private key")}: {data.privateKey}</p>
          <p>WIF: {data.wif}</p>
          <p>{t("wallet.public key")}：{data.publicKey}</p>
        </div>
      ),
      okText: t("button.ok")
    });
  }
  render() {
    const { assetlist, address } = this.state;
    const { t } = this.props;
    const { location } = this.props.router;
    const accountType = location.state?.account.accountType ?? 2;
    return (
      <Layout className="gui-container wa-detail">

        <Topath topath={this.state.topath}></Topath>
        <Sync />

        <Content className="mt3">
          <Row gutter={[30, 0]}>
            <Col span={24} className="bg-white pv4">
              <PageHeader title={t("wallet.account")}></PageHeader>
              <List
                header={<div>{address}</div>}
                footer={<span></span>}
                itemLayout="horizontal"
                dataSource={assetlist.balances}
                renderItem={item => (
                  <List.Item className="wa-half">
                    <Typography.Text className="font-s">
                      <span className="upcase">{item.symbol}</span>
                      <span>{item.balance}</span>
                    </Typography.Text>
                  </List.Item>
                )}
              />
              <div className="mb4 text-r">
                <Button type="primary" onClick={this.showPrivate} style={{ visibility: (accountType === 2 ? 'hidden' : 'default') }}>{t("button.show details")}</Button>
                <Button className="ml3" onClick={this.deleteConfirm}>{t("button.delete account")}</Button>
              </div>
            </Col>
          </Row>

          <Row gutter={[30, 0]} className="mt2 mb2" type="flex" style={{ 'minHeight': '120px' }}>
            <Col span={24} className="bg-white pv4">
              <PageHeader title={t("blockchain.transactions")}></PageHeader>
              <Transaction page="walletdetail" content={t("wallet.transactions")} />
            </Col>
          </Row>
        </Content>
      </Layout>
    );
  }
}

export default Walletdetail;