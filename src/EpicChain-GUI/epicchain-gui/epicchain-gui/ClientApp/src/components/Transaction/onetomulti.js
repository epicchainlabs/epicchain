/* eslint-disable */
import React, { createRef } from 'react';
import {
    Alert, Input,
    Form,
    InputNumber,
    Modal,
    Select,
    Row,
    Col,
    message,
    Button,
    AutoComplete,
} from 'antd';
import '../../static/css/wallet.css'
import { withTranslation, Trans } from "react-i18next";
import { post } from "../../core/request";

const { Option } = Select;

@withTranslation()
class Onetomulti extends React.Component {
    constructor(props) {
        super(props);
        this.myForm = createRef();
        this.state = {
            iconLoading: false,
            selectadd: []
        };
    }
    setAddress = target => {
        target = target ? target : 0;
        let _detail = this.props.account[target].balances;
        this.setState({
            selectadd: _detail
        })
    }
    clickToCopy = (text) => {
        navigator.clipboard.writeText(text)
        message.success(<Trans>common.copied</Trans>)
    }
    transfer = values => {
        var _this = this;
        const { t } = this.props;
        if (values.receiver.trim() === "") {
            message.error(t("wallet.payment empty"));
            return false;
        }

        var receivers = [];
        var addlist = values.receiver.trim().split(/\n+/);
        addlist.map(item => {
            let _item = item.trim().split(/\s+/);
            _item.filter(cur => { return cur !== null && cur !== undefined; })
            if (_item.length !== 2) { message.error(t("wallet.input error")); return false; }

            let receiver = {};
            receiver.address = _item[0];
            receiver.amount = _item[1];
            receivers.push(receiver);
        })

        var params = {
            "sender": this.props.account[values.sender].address,
            "receivers": receivers,
            "asset": values.asset
        };

        post("SendToMultiAddress", params).then(res => {
            const _data = res.data;
            const result = res.data.result;
            if (_data.msgType === -1) {
                let res = _data.error;
                let title = (<Trans>wallet.transfer send error</Trans>);
                let content = (
                    <div className="show-pri">
                        <p><Trans>blockchain.transaction hash</Trans>: {res.code}</p>
                        <p><Trans>error msg</Trans>: {res.message}</p>
                    </div>
                );

                if (res.code === 20014) {
                    title = (<Trans>wallet.transfer send error 20014</Trans>);
                    content = (
                        <div className="show-pri">
                            <pre style={{ overflow: 'hidden', overflowX: 'auto', overflowY: 'scroll', maxHeight: '60vh', width: 'auto' }}>
                                <code>{JSON.stringify(JSON.parse(res.message), null, 2)}</code>
                            </pre>
                            <p>
                                <Button type="link" style={{ margin: 0, color: '#00B594' }} onClick={() => _this.clickToCopy(res.message)}>
                                    <Trans>button.copy to clipboard</Trans>
                                </Button>
                            </p>
                        </div>
                    );
                }
                const args = {
                    title: title,
                    width: 650,
                    centered: true,
                    content: content,
                    okText: (<Trans>button.confirm</Trans>)
                };
                if (res.code === 20014) {
                    Modal.warning(args);
                } else {
                    Modal.error(args);
                }
                return;
            } else {
                Modal.success({
                    title: t('wallet.transfer send success'),
                    content: (
                        <div className="show-pri">
                            <p>{t("blockchain.transaction hash")}：</p>
                            <p>{result.txId}</p>
                        </div>
                    ),
                    okText: t("button.confirm")
                });
                this.myForm.current.resetFields()
                this.setState({
                    selectadd: []
                })
            }
        })
    }
    render() {
        const { account } = this.props;
        const { selectadd } = this.state;
        const { t } = this.props;
        return (
            <Form ref="formRef" className="trans-form" onFinish={this.transfer}>
                <Row gutter={[30, 0]} className="bg-white pv4">
                    <Col span={24}>
                        <div className="w600 mt3" style={{ 'minHeight': 'calc( 100vh - 350px )' }}>
                            <Row>
                                <Col span={15}>
                                    <Form.Item
                                        name="sender"
                                        label={t("wallet.from")}
                                        rules={[
                                            {
                                                required: true,
                                                message: t("wallet.please select a account"),
                                            },
                                        ]}
                                    >
                                        <Select
                                            placeholder={t("select account")}
                                            style={{ width: '100%' }}
                                            onChange={this.setAddress}>
                                            {account.map((item, index) => {
                                                return (
                                                    <Option key={index}>{item.address}</Option>
                                                )
                                            })}
                                        </Select>
                                    </Form.Item>
                                </Col>
                                <Col span={9}>
                                    <Form.Item
                                        name="asset"
                                        label={t("wallet.asset")}
                                        rules={[
                                            {
                                                required: true,
                                                message: t("wallet.required"),
                                            },
                                        ]}>
                                        <Select
                                            placeholder={t("wallet.select")}
                                            style={{ width: '100%' }}
                                        >
                                            {selectadd.map((item, index) => {
                                                return (
                                                    <Option key={index} value={item.asset} title={item.symbol}><span className="trans-symbol">{item.symbol} </span><small>{item.balance}</small></Option>
                                                )
                                            })}
                                        </Select>
                                    </Form.Item>
                                </Col>
                            </Row>

                            <Form.Item
                                name="receiver"
                                label={t("wallet.to")}
                                rules={[
                                    {
                                        required: true,
                                        message: t("wallet.required"),
                                    },
                                ]}
                            >
                                <Input.TextArea placeholder={t("wallet.payment bulk")} />
                            </Form.Item>
                            <div className="text-c lighter">
                                <small>{t("wallet.estimated time")}：12s </small>
                            </div>
                            <Form.Item>
                                <Button type="primary" htmlType="submit" loading={this.state.iconLoading}>
                                    {t("button.send")}
                                </Button>
                            </Form.Item>
                        </div>
                    </Col>
                </Row>
            </Form>
        );
    }
}

export default Onetomulti;