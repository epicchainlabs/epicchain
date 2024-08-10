/* eslint-disable */
import React, { useState } from 'react';
import "../../static/css/advanced.css";
import '../../static/css/trans.css';
import { Layout, Tabs, message, Row, Col, Modal, Button, Input, Select, Form, InputNumber, Tag } from 'antd';
import { Statistic } from 'antd';
import Sync from '../sync';
import { observer, inject } from "mobx-react";
import { useTranslation, withTranslation, Trans } from "react-i18next";
import { post } from "../../core/request";
import { withAuthenticated } from '../../core/authentication';
import { WalletOutlined, RetweetOutlined, ForkOutlined, EditOutlined } from '@ant-design/icons';
import { Copy } from '../copy';

const { Option } = Select;
const { Content } = Layout;

@withAuthenticated
@withTranslation()
@inject("walletStore")
@observer
class AdvancedCommittee extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      visible: false,
      show: false,
      accounts: [],
      locked: true,
      showlocked: false,
      title: "111",
      transnum: 0,
      blocksize: 0,
      blockfee: 0,
      bytefee: 0,
    };
  }
  componentDidMount() {
    this.getMaxTrans();
    this.getBlocksize();
    this.getBlockFee();
    this.getByteFee();
  }
  showDrawer = () => {
    this.setState({
      visible: true,
    });
  };
  hideModal = () => {
    this.setState({
      visible: false,
    });
  };
  handleCancel = () => {
    this.setState({
      visible: false,
    });
  };
  changeDialog = (ele) => {
    const { t } = this.props;
    return () => {
      const accounts = Array.call([], ...this.props.walletStore.accountlist);
      switch (ele) {
        case 0:
          this.setState({
            title: t("advanced.com-trans"),
            children: <TransNumber account={accounts} func={this.handleCancel} />,
          });
          break;
        case 1:
          this.setState({
            title: t("advanced.com-blocksize"),
            children: <BlockSize account={accounts} func={this.handleCancel} />,
          });
          break;
        case 2:
          this.setState({
            title: t("advanced.com-blockfee"),
            children: <BlockFee account={accounts} func={this.handleCancel} />,
          });
          break;
        case 3:
          this.setState({
            title: t("advanced.com-bytefee"),
            children: <ByteFee account={accounts} func={this.handleCancel} />,
          });
          break;
        case 4:
          this.setState({
            title: t("advanced.com-account"),
            children: <AccountState account={accounts} func={this.handleCancel} />,
          });
          break;
      }
      this.setState({
        visible: true,
      });
    };
  }
  getMaxTrans = () => {
    post("GetMaxTransactionsPerBlock", {}).then(result => {
      this.setState({
        transnum: result.data.result,
      });
    });
  }
  getBlocksize = () => {
    post("GetMaxBlockSize", {}).then(result => {
      this.setState({
        blocksize: result.data.result,
      });
    });
  }
  getBlockFee = () => {
    post("GetMaxBlockSystemFee", {}).then(result => {
      this.setState({
        blockfee: result.data.result,
      });
    });
  }
  getByteFee = () => {
    post("GetFeePerByte", {}).then(result => {
      this.setState({
        bytefee: result.data.result,
      });
    });
  }
  searchAdd = ({ target: { value } }) => {
    const { t } = this.props;
    if (value.length <= 0) return;

    this.setState({
      showlocked: false
    });

    const param = { "account": value };
    var regex = new RegExp("^[N][1-9A-HJ-NP-Za-km-z]{32,34}$");
    if (!regex.test(value)) {
      message.error(t("advanced.com-veri-account"));
      return;
    }

    post("IsBlocked", param).then(result => {
      console.log(result.data)
      if (result.data.msgType == 3) {
        this.setState({
          locked: result.data.result,
          showlocked: true
        });
      }
    });
  }
  render() {
    const { t } = this.props;
    const { transnum, blocksize, blockfee, bytefee, locked, showlocked } = this.state;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row gutter={[30, 0]} style={{ minHeight: "calc( 100vh - 120px )" }}>
            <Col span={24} className="bg-white pv4">
              <Tabs className="committe-title" defaultActiveKey="1"
                items={[
                  {
                    label: t("advanced.com-bytefee"), key: 4, children: (
                      <div><Statistic title={t("advanced.com-bytefee-set")} value={bytefee} prefix={<RetweetOutlined />} />
                        <Button className="mt3" type="primary" onClick={this.changeDialog(3)}>{t("advanced.modify")}</Button>
                      </div>)
                  },
                  {
                    label: t("advanced.com-account"), key: 5, children: (
                      <div>
                        <h4 className="bolder mb4">{t("advanced.com-input")}</h4>
                        <Input
                          placeholder="NLGMSsGTDsLbAfGCBJvNmUMj16kvHHjFpa"
                          prefix={<WalletOutlined />}
                          onBlur={this.searchAdd} />
                        {showlocked ? <div className="mt4">
                          <span className="para-tag">
                            {t("advanced.com-account-state")}
                            {locked ? <em>{t("advanced.com-locked")}</em> : <em>{t("advanced.com-unlocked")}</em>}
                          </span>
                        </div>
                          : null}
                        <Button className="mt3" type="primary" onClick={this.changeDialog(4)}>{t("advanced.modify")}</Button>
                      </div>)
                  }
                ]}
              />
            </Col>
          </Row>
        </Content>
        <Modal
          className="set-modal"
          title={<Trans>{this.state.title}</Trans>}
          open={this.state.visible}
          onCancel={this.hideModal}
          footer={null}
        >
          {this.state.children}
        </Modal>
      </Layout>
    );
  }
}

export default AdvancedCommittee;

const TransNumber = ({ account, func }) => {
  const [form] = Form.useForm();
  const { t } = useTranslation();
  const setTrans = values => {
    console.log(values)
    let params = {
      "max": values.max,
      "signers": values.signers
    };
    post("SetMaxTransactionsPerBlock", params).then(res => {
      var _data = res.data;
      if (_data.msgType === -1) {
        ModalError(_data, t("advanced.com-set-fail"));
      } else {
        ModalSuccess(_data, t("advanced.com-set-success"))
        form.resetFields();
        func();
      }
    }).catch(function (error) {
      console.log(error);
    });
  }
  if (account.length === 0) return null;
  return (
    <div className="w400">
      <Form className="neo-form" form={form} onFinish={setTrans}>
        <h4>{t("advanced.com-set-max")}</h4>
        <Form.Item name="max" rules={[{ required: true, message: t("advanced.com-input-max") }]}>
          <InputNumber
            placeholder={t("advanced.com-set-max")}
            parser={value => value.replace(/[^0-9]/g, '')}
            step={1} max={65534}
            style={{ width: '100%' }} />
        </Form.Item>
        <h4>{t("advanced.com-select-add")}</h4>
        <Form.Item name="signers" rules={[{ required: true, message: t("advanced.com-select-add") }]}>
          <Select
            placeholder={t("advanced.com-select-add")}
            mode="tags"
            className="multiadd"
            style={{ width: '100%' }}>
            {account.length > 0 ? account.map((item) => {
              return (
                <Option key={item.address}>{item.address}</Option>
              )
            }) : null}
          </Select>
        </Form.Item>
        <Form.Item>
          <Button style={{ 'width': '100%' }} type="primary" htmlType="submit">{t("button.confirm")}</Button>
        </Form.Item>
      </Form>
    </div>
  )
};

const BlockSize = ({ account, func }) => {
  const [form] = Form.useForm();
  const { t } = useTranslation();
  const setTrans = values => {
    console.log(values)
    let params = {
      "max": values.max,
      "signers": values.signers
    };
    post("SetMaxBlockSize", params).then(res => {
      var _data = res.data;
      if (_data.msgType === -1) {
        ModalError(_data, t("advanced.com-set-fail"));
      } else {
        ModalSuccess(_data, t("advanced.com-set-success"))
        form.resetFields();
        func();
      }
    }).catch(function (error) {
      console.log(error);
    });
  }
  if (account.length === 0) return null;
  return (
    <div className="w400">
      <Form className="neo-form" form={form} onFinish={setTrans}>
        <h4>{t("advanced.com-set-max")}</h4>
        <Form.Item name="max" rules={[{ required: true, message: t("advanced.com-input-max") }]}>
          <InputNumber
            placeholder={t("advanced.com-set-max")}
            parser={value => value.replace(/[^0-9]/g, '')}
            step={1} max={33554432}
            style={{ width: '100%' }} />
        </Form.Item>
        <h4>{t("advanced.com-select-add")}</h4>
        <Form.Item name="signers" rules={[{ required: true, message: t("advanced.com-select-add") }]}>
          <Select
            placeholder={t("advanced.com-select-add")}
            mode="tags"
            className="multiadd"
            style={{ width: '100%' }}>
            {account.length > 0 ? account.map((item) => {
              return (
                <Option key={item.address}>{item.address}</Option>
              )
            }) : null}
          </Select>
        </Form.Item>
        <Form.Item>
          <Button style={{ 'width': '100%' }} type="primary" htmlType="submit">{t("button.confirm")}</Button>
        </Form.Item>
      </Form>
    </div>
  )
};

const BlockFee = ({ account, func }) => {
  const [form] = Form.useForm();
  const { t } = useTranslation();
  const setTrans = values => {
    let params = {
      "max": values.max,
      "signers": values.signers
    };
    post("SetMaxBlockSystemFee", params).then(res => {
      var _data = res.data;
      if (_data.msgType === -1) {
        ModalError(_data, t("advanced.com-set-fail"));
      } else {
        ModalSuccess(_data, t("advanced.com-set-success"))
        form.resetFields();
        func();
      }
    }).catch(function (error) {
      console.log(error);
    });
  }
  if (account.length === 0) return null;
  return (
    <div className="w400">
      <Form className="neo-form" form={form} onFinish={setTrans}>
        <h4>{t("advanced.com-set-max")}</h4>
        <Form.Item name="max" rules={[{ required: true, message: t("advanced.com-input-max") }]}>
          <InputNumber
            placeholder={t("advanced.com-set-max")}
            parser={value => value.replace(/[^0-9]/g, '')}
            step={1} min={4007600}
            style={{ width: '100%' }} />
        </Form.Item>
        <h4>{t("advanced.com-select-add")}</h4>
        <Form.Item name="signers" rules={[{ required: true, message: t("advanced.com-select-add") }]}>
          <Select
            placeholder={t("advanced.com-select-add")}
            mode="tags"
            className="multiadd"
            style={{ width: '100%' }}>
            {account.length > 0 ? account.map((item) => {
              return (
                <Option key={item.address}>{item.address}</Option>
              )
            }) : null}
          </Select>
        </Form.Item>
        <Form.Item>
          <Button style={{ 'width': '100%' }} type="primary" htmlType="submit">{t("button.confirm")}</Button>
        </Form.Item>
      </Form>
    </div>
  )
};

const ByteFee = ({ account, func }) => {
  const [form] = Form.useForm();
  const { t } = useTranslation();
  const setTrans = values => {
    console.log(values)
    let params = {
      "fee": values.fee,
      "signers": values.signers
    };
    post("SetFeePerByte", params).then(res => {
      var _data = res.data;
      console.log(_data);
      if (_data.msgType === -1) {
        ModalError(_data, t("advanced.com-set-fail"));
      } else {
        ModalSuccess(_data, t("advanced.com-set-success"))
        form.resetFields();
        func();
      }
    }).catch(function (error) {
      console.log(error);
    });
  }
  if (account.length === 0) return null;
  return (
    <div className="w400">
      <Form className="neo-form" form={form} onFinish={setTrans}>
        <h4>{t("advanced.com-set-max")}</h4>
        <Form.Item name="fee" rules={[{ required: true, message: t("advanced.com-input-max") }]}>
          <InputNumber
            placeholder={t("advanced.com-set-max")}
            parser={value => value.replace(/[^0-9]/g, '')}
            step={1} max={33554432}
            style={{ width: '100%' }} />
        </Form.Item>
        <h4>{t("advanced.com-select-add")}</h4>
        <Form.Item name="signers" rules={[{ required: true, message: t("advanced.com-select-add") }]}>
          <Select
            placeholder={t("advanced.com-select-add")}
            mode="tags"
            className="multiadd"
            style={{ width: '100%' }}>
            {account.length > 0 ? account.map((item) => {
              return (
                <Option key={item.address}>{item.address}</Option>
              )
            }) : null}
          </Select>
        </Form.Item>
        <Form.Item>
          <Button style={{ 'width': '100%' }} type="primary" htmlType="submit">{t("button.confirm")}</Button>
        </Form.Item>
      </Form>
    </div>
  )
};


const AccountState = ({ account, func }) => {
  const [form] = Form.useForm();
  const { t } = useTranslation();
  const [locked, changelocked] = useState(true);
  const [show, changeShow] = useState(false);
  const onBlur = () => {
    var add = form.getFieldValue().account || "";
    if (add.length <= 0) return;

    var regex = new RegExp("^[N][1-9A-HJ-NP-Za-km-z]{32,34}$");
    if (!regex.test(add)) return;

    const param = { "account": add };
    post("IsBlocked", param).then(result => {
      changelocked(result.data.result)
    });
  }
  const setTrans = values => {
    console.log(values)
    let params = {
      "account": values.account,
      "signers": values.signers
    };
    post("BlockAccount", params).then(res => {
      var _data = res.data;
      if (_data.msgType === -1) {
        ModalError(_data, t("advanced.com-set-fail"));
      } else {
        ModalSuccess(_data, t("advanced.com-set-success"))
        form.resetFields();
        func();
      }
    }).catch(function (error) {
      console.log(error);
    });
  }
  if (account.length === 0) return null;
  return (
    <div className="w400">
      <Form className="neo-form" form={form} onFinish={setTrans}>
        <h4>{t("advanced.com-account")}</h4>
        <Form.Item
          name="account"
          rules={[{
            pattern: "^[N][1-9A-HJ-NP-Za-km-z]{32,34}$",
            message: t("wallet.address format"),
          }, {
            required: true,
            message: t("advanced.com-input")
          }]}>
          <Input
            placeholder={t("advanced.com-input")}
            onBlur={onBlur}
            style={{ width: '100%' }} />
        </Form.Item>
        <h4>{t("advanced.com-select-add")}</h4>
        <Form.Item name="signers" rules={[{ required: true, message: t("advanced.com-select-add") }]}>
          <Select
            placeholder={t("advanced.com-select-add")}
            mode="tags"
            className="multiadd"
            style={{ width: '100%' }}>
            {account.length > 0 ? account.map((item) => {
              return (
                <Option key={item.address}>{item.address}</Option>
              )
            }) : null}
          </Select>
        </Form.Item>
        <Form.Item>
          {locked ?
            <Button type="primary" htmlType="submit" style={{ width: '100%' }}>{t("advanced.com-unlocked-account")}</Button> :
            <Button type="primary" htmlType="submit" style={{ width: '100%' }}>{t("advanced.com-locked-account")}</Button>}
        </Form.Item>
      </Form>
    </div>
  )
};

const ModalError = (data, title) => {
  Modal.error({
    width: 600,
    title: title,
    content: (
      <div className="show-pri">
        <p><Trans>error</Trans> ：{data.error.message} <Copy msg={data.error.message} /></p>
      </div>
    ),
    okText: <Trans>button.ok</Trans>
  });
};

const ModalSuccess = (data, title) => {
  Modal.success({
    width: 600,
    title: title,
    content: (
      <div className="show-pri">
        <p><Trans>hash</Trans> ：{data.result.txId}</p>
      </div>
    ),
    okText: <Trans>button.ok</Trans>
  });
};