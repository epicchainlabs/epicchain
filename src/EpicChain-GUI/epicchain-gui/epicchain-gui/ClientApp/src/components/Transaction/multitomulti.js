/* eslint-disable */
import React, { createRef } from 'react';
import {
    Modal, Input,
    Form,
    InputNumber,
    Select,
    Row,
    Col,
    message,
    Divider,
    Button,
} from 'antd';
import '../../static/css/wallet.css'
import { withTranslation, Trans } from "react-i18next";
import { post } from "../../core/request";
import { MinusSquareOutlined, PlusOutlined } from '@ant-design/icons';
const { Option } = Select;


@withTranslation()
class Multitomulti extends React.Component {
    constructor(props) {
        super(props);
        this.myForm = createRef();
        this.state = {
            iconLoading: false,
            assetlist: []
        };
    }
    setAddress = (formIndexKey, address) => {
        var acct = this.props.account.find(a => a.address == address);
        if (acct) {
            let list = this.state.assetlist;
            list[formIndexKey] = acct.balances;
            this.setState({
                assetList: list
            });
        } else {
            console.error(address, " not exits");
        }
    }

    clickToCopy = (text) => {
        navigator.clipboard.writeText(text)
        message.success(<Trans>common.copied</Trans>)
    }
    transfer = values => {
        const { t } = this.props;
        var _this = this;
        this.setState({ iconLoading: true });
        post("SendTo", values.params).then(res => {
            var _data = res.data;
            var result = res.data.result;
            _this.setState({ iconLoading: true });

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
                                <Button type="link" style={{ margin: 0, color: '#00B594' }} onClick={() => this.clickToCopy(res.message)}>
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
                    assetlist: []
                })
            }
        })
    }
    render() {
        const { account, t } = this.props;
        const { assetlist } = this.state;
        return (
            <div className="w600 info-detail mt3">
                <Form ref={this.myForm} className="trans-form" onFinish={this.transfer} initialValues={{ params: [null] }}>
                    <Form.List name="params">
                        {(fields, { add, remove }) => {
                            return (
                                <div>
                                    {fields.map((field) => (
                                        <div key={field.key}>
                                            <Row>
                                                <Col span="15">
                                                    <Form.Item
                                                        name={[field.name, "sender"]}
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
                                                            onChange={(e) => this.setAddress(field.key, e)}>
                                                            {account.map((item, index) => {
                                                                return (
                                                                    <Option key={item.address}>{item.address}</Option>
                                                                )
                                                            })}
                                                        </Select>
                                                    </Form.Item>
                                                </Col>
                                                <Col span="9">
                                                    <Form.Item
                                                        name={[field.name, "asset"]}
                                                        label={t("wallet.asset")}
                                                        rules={[
                                                            {
                                                                required: true,
                                                                message: t("wallet.required"),
                                                            },
                                                        ]}
                                                    >
                                                        <Select
                                                            placeholder={t("wallet.select")}
                                                            style={{ width: '100%' }}>
                                                            {!!assetlist[field.key] ? assetlist[field.key].map((item) => {
                                                                return (
                                                                    <Option key={item.asset}><span className="trans-symbol">{item.symbol} </span><small>{item.balance}</small></Option>
                                                                )
                                                            }) : null}
                                                        </Select>
                                                    </Form.Item>
                                                </Col>
                                            </Row>
                                            <Row>
                                                <Col span="15">
                                                    {/*  pattern={value => value.replace(/[^0-9]/g, '')} */}
                                                    <Form.Item
                                                        name={[field.name, "receiver"]}
                                                        label={t("wallet.to")}
                                                        rules={[{
                                                            pattern: "^[N][1-9A-HJ-NP-Za-km-z]{32,34}$",
                                                            message: t("wallet.address format"),
                                                        },
                                                        {
                                                            required: true,
                                                            message: t("wallet.please input a correct amount"),
                                                        },
                                                        ]}
                                                    >
                                                        <Input placeholder={t("input account")} />
                                                    </Form.Item>
                                                </Col>
                                                <Col span="9">
                                                    <Form.Item
                                                        name={[field.name, "amount"]}
                                                        label={t("wallet.amount")}
                                                        rules={[
                                                            {
                                                                required: true,
                                                                message: t("wallet.required"),
                                                            },
                                                        ]}>
                                                        <InputNumber min={0} step={1} placeholder={t("wallet.amount")} />
                                                    </Form.Item>
                                                </Col>
                                                {fields.length > 1 ? (
                                                    <Divider orientation="right">
                                                        <a className="delete-line" onClick={() => { remove(field.name); }}><MinusSquareOutlined /> <span className="font-s">{t("wallet.delete add")}</span></a>
                                                    </Divider>
                                                ) : null}
                                            </Row>
                                        </div>
                                    ))}
                                    <Form.Item className="mb0">
                                        <Button
                                            type="dashed"
                                            onClick={() => {
                                                add();
                                            }}
                                            style={{ width: "100%" }}
                                        >
                                            <PlusOutlined /> {t("wallet.transfer add")}
                                        </Button>
                                    </Form.Item>
                                </div>
                            );
                        }}
                    </Form.List>

                    <Form.Item>
                        <Button type="primary" htmlType="submit">
                            {t("wallet.transfer")}
                        </Button>
                    </Form.Item>
                </Form>
            </div>
        );
    }
}

export default Multitomulti;