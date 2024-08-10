/* eslint-disable */
import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { observer, inject } from "mobx-react";
import { Layout, message, Row, Col, List, Avatar, Button, Typography, PageHeader, Modal, Input, Select, Form, InputNumber } from 'antd';
import '../../static/css/wallet.css'
import Sync from '../sync';
import { withTranslation, useTranslation, Trans } from "react-i18next";
import { post, postAsync } from "../../core/request";
import { withAuthenticated } from '../../core/authentication';
import { Copy } from '../copy';

import {
  PlusCircleOutlined
} from '@ant-design/icons';

const { Content } = Layout;
const { Option } = Select;


@withAuthenticated
@withTranslation()
@inject("walletStore")
@observer
class Walletlist extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      size: 'default',
      assetlist: [],
      iconLoading: false
    };
  }
  componentDidMount() {
    this.getAddress();
    this.getAllasset();
    this.getGas();
  }
  getAllasset = async () => {
    let data = await postAsync("GetMyTotalBalance", {});
    if (data.msgType === -1) {
      console.log("GetMyTotalBalance Error");
      console.log(data);
      return;
    }
    this.setState({
      assetlist: data.result
    });
  }
  getAddress = async () => {
    let data = await postAsync("ListAddress", {
      "count": 10
    });
    if (data.msgType === -1) {
      console.log("ListAddress Error");
      console.log(data);
      return;
    }
    this.props.walletStore.setAccounts(data.result.accounts);
  }
  getGas = async () => {
    let data = await postAsync("ShowGas");
    if (data.msgType == -1) {
      console.log("ShowGas Error");
      console.log(data);
      return;
    }
    this.props.walletStore.setUnclaimedGas(data.result.unclaimedGas);
  }
  claimGas = async () => {
    const { t } = this.props;
    this.setState({ iconLoading: true })
    setTimeout(() => { this.setState({ iconLoading: false }); this.getGas() }, 15000);

    let data = await postAsync("ClaimGas");
    if (data.msgType === -1) {
      message.success(t("wallet.gas fail"), 3);
      return;
    }
    if (data.msgType = 3) {
      message.success(t("wallet.gas success"), 3);
    }
  }
  addAddress = async () => {
    const { t } = this.props;
    let data = await postAsync("CreateAddress");
    if (data.msgType === -1) {
      message.error(t('wallet.open wallet first'));
      console.log(data)
      return;
    }
    message.success(t('wallet.add address success'));
    this.props.walletStore.addAccount(data.result);
  }
  showModal = (ele) => {
    const { t } = this.props;
    return () => {
      this.setState({ visible: true })
      switch (ele) {
        case 0: this.setState({ modalPanel: <Private func={this.handleCancel} />, modalTitle: t("wallet.import private") }); break;
        case 1: this.setState({ modalPanel: <Multiaddress func={this.handleCancel} />, modalTitle: t("wallet.signature multi") }); break;
        default: this.setState({ visible: false }); break;
      }
    }
  };
  handleOk = () => {
    this.setState({
      visible: false,
    });
  };
  handleCancel = () => {
    this.setState({
      visible: false,
    });
  };
  render() {
    const accounts = this.props.walletStore.accountlist;
    const unclaimedGas = this.props.walletStore.unclaimedGas;
    const { assetlist } = this.state;
    const { t } = this.props;

    let unnoadd = [], normaladd = [], mutiadd = [], contractadd = [];
    Array.call([], ...accounts).map(function (item) {
      let _item = { ...item };
      switch (_item.accountType) {
        case 0: unnoadd.push(_item); break;
        case 1: normaladd.push(_item); break;
        case 2: mutiadd.push(_item); break;
        case 3: contractadd.push(_item); break;
      }
    });

    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row className="mb2" gutter={[30, 0]} type="flex" style={{ 'minHeight': 'calc( 100vh - 120px )' }}>
            <Col span={13} className="bg-white pv4">
              <div className="in-title">
                <h2 className="mb0">
                  {t("wallet.accounts")}
                  <div className="wal-import float-r">
                    <PlusCircleOutlined className="" />
                    <div className="wal-ul">
                      <ul>
                        <li><a onClick={this.addAddress}>{t('wallet.add address')}</a></li>
                        <li><a onClick={this.showModal(0)}>{t('wallet.import private')}</a></li>
                        <li><a onClick={this.showModal(1)}>{t('wallet.signature multi')}</a></li>
                      </ul>
                    </div>
                  </div>
                </h2>
              </div>
              <Accounts accounts={normaladd} name={t("wallet.address standard")} />
              <Accounts accounts={mutiadd} name={t("wallet.address multi sign")} />
              <Accounts accounts={contractadd} name={t("wallet.address contract")} />
              <Accounts accounts={unnoadd} name={t("wallet.address non")} />
            </Col>
            <Col span={10} offset={1} className="bg-white pv4">
              <PageHeader title={t("wallet.assets")} ></PageHeader>
              <List
                className="asset-list"
                itemLayout="horizontal"
                style={{ 'minHeight': 'calc( 100% - 135px )' }}
                dataSource={assetlist}
                header={<div><span>{t("blockchain.asset info")} <small></small></span><span className="float-r wa-amount">{t("wallet.balance")}</span></div>}
                renderItem={item => (
                  <List.Item>
                    <List.Item.Meta
                      avatar={
                        <Avatar src={"https://neo.org/images/gui/" + item.asset + ".png"} />
                      }
                      title={<span className="upcase" title={item.asset}>{item.symbol}</span>}
                    // description={<span className="f-xs">{item.asset}</span>}
                    />
                    <Typography>{item.balance}</Typography>
                  </List.Item>
                )}
              />
              <div className="w200 mt4">
                <Button className="w200" onClick={this.claimGas} loading={this.state.iconLoading}>{t("button.claim")} {unclaimedGas} GAS</Button>
              </div>
            </Col>
          </Row>
          <Modal
            width={400}
            centered
            title={this.state.modalTitle}
            open={this.state.visible}
            onCancel={this.handleCancel}
            footer={null}
          >
            {this.state.modalPanel}
          </Modal>
        </Content>
      </Layout>
    );
  }
}

export default Walletlist;



const Private = ({ func }) => {
  const [form] = Form.useForm();
  const { t } = useTranslation();
  const importPrivate = values => {
    post("ImportAccounts", [values.private]).then(res => {
      let _data = res.data;
      if (_data.msgType === 3) {
        message.success(t('wallet.import private success'), 2);
      } else {
        message.info(t('wallet.private fail'), 2);
      }
      func();
    }).catch(function (error) {
      console.log("error");
      console.log(error);
    });
  }
  return (
    <Form className="neo-form" form={form} onFinish={importPrivate}>
      <Form.Item name="private" rules={[{ required: true, message: t("please input Hex/WIF private key") }]}>
        <Input placeholder={t("please input Hex/WIF private key")} />
      </Form.Item>
      <Form.Item>
        <Button type="primary" htmlType="submit">{t("wallet.import private")}</Button>
      </Form.Item>
    </Form>
  )
};

const Multiaddress = ({ func }) => {
  const [form] = Form.useForm();
  const { t } = useTranslation();
  const [accounts, changeList] = useState([]);
  const [maxnum, changeNum] = useState(1);
  const [sigBTdisabled, changeSigBTdisabled] = useState(false);

  const getPublic = () => {
    //列出所有可选举公钥
    post("ListCandidatePublicKey", {}).then(res => {
      var _data = res.data;
      if (_data.msgType === -1) {
        message.error(t("alert msg.no find"));
      } else {
        changeList(_data.result);
      }
      return;
    }).catch(function (error) {
      console.log(error);
    });
  }
  const addMulti = values => {
    let params = {
      "limit": values.limit,
      "publicKeys": values.publicKeys
    };
    post("CreateMultiAddress", params).then(res => {
      var _data = res.data;
      if (_data.msgType === -1) {
        message.error(<Trans>wallet.signature multi error</Trans>);
        return;
      } else {
        func();
        Modal.success({
          width: 600,
          title: <Trans>wallet.signature multi success</Trans>,
          content: (
            <div className="show-pri">
              <p><Trans>hash</Trans> ：{_data.result.scriptHash}</p>
              <p><Trans>wallet.address multi sign</Trans> ：{_data.result.address}</p>
            </div>
          ),
          okText: <Trans>button.ok</Trans>
        });
        form.resetFields();
      }
    }).catch(function (error) {
      console.log(error);
    });
  }
  const handleChange = value => {
    if (value.length <= 0) return;
    let last = value.pop().trim();
    var regex = new RegExp("^0[23][0-9a-f]{64}$");
    if (!regex.test(last)) {
      message.error(t("wallet.public key error"));
      return;
    }
    value.push(last);
    changeNum(value.length);
  }
  const handleSignatureMinChange = value => {
    if (value < 0 || value > maxnum) {
      changeSigBTdisabled(true);
      message.error(t("wallet.signature max input"));
    } else {
      changeSigBTdisabled(false);
    }
  }
  if (accounts.length === 0) getPublic();
  return (
    <Form className="neo-form" form={form} onFinish={addMulti}>
      {console.log(accounts)}
      <h4>{t("wallet.signature multi create")}</h4>
      <Form.Item name="publicKeys" rules={[{ required: true, message: t("wallet.please input public key") }]}>
        <Select
          placeholder={t("wallet.signature public")}
          mode="tags"
          onChange={handleChange}
          className="multiadd"
          style={{ width: '100%' }}>
          {accounts.length > 0 ? accounts.map((item) => {
            console.log({ ...item })
            return (
              <Option className="add-list" key={item.publicKey}>{item.publicKey}<span className="add-show">{item.address}</span></Option>
            )
          }) : null}
        </Select>
      </Form.Item>
      <h4>{t("wallet.signature min")}</h4>
      <Form.Item name="limit" rules={[{ required: true, message: t("wallet.please input signature min") }]}>
        <InputNumber
          placeholder={t("wallet.signature min input")}
          parser={value => value.replace(/[^0-9]/g, '')}
          step={1} min={1}
          onChange={handleSignatureMinChange}
          style={{ width: '100%' }} />
      </Form.Item>
      <Form.Item>
        <Button style={{ 'width': '100%' }} type="primary" htmlType="submit" disabled={sigBTdisabled}>{t("button.confirm")}</Button>
      </Form.Item>
    </Form>
  )
}

const Accounts = ({ accounts, name }) => {
  const { t } = useTranslation();
  if (accounts.length === 0) return null;
  return (
    <List
      itemLayout="horizontal"
      dataSource={accounts}
      header={<div>{name}</div>}
      renderItem={item => (
        <List.Item>
          <List.Item.Meta
            title={<div>
              <Link to={{
                pathname: "/wallet/walletlist:" + item.address,
                state: { account: item }
              }}
                state={{ account: item }}
                title={t("wallet.show detail")} >{item.address}</Link>
              <Copy msg={item.address} />
            </div>}
            description={
              <span className="f-s">
                <span className="amount mr2">NEO <span className="wa-count">{item.neo}</span></span>
                <span>GAS <span className="wa-count">{item.gas}</span></span>
              </span>}
          />
        </List.Item>
      )}
    />
  )
}