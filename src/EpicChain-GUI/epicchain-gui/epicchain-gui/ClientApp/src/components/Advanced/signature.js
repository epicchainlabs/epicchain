/* eslint-disable */
import React, { createRef } from 'react';
import 'antd/dist/antd.min.css';
import { Form, Input, Button } from 'antd';
import { Layout, Row, Col, Tabs, message, PageHeader, Modal } from 'antd';
import Sync from '../sync';
import "../../static/css/advanced.css";
import { withTranslation } from "react-i18next";
import { Trans } from 'react-i18next';
import { post } from "../../core/request";
import { withAuthenticated } from '../../core/authentication';


const { Content } = Layout;
const { TextArea } = Input;

function clickToCopy(text) {
    navigator.clipboard.writeText(text)
    message.success(<Trans>common.copied</Trans>)
}

function success(data) {
    const { method, result } = data;
    let title = (<Trans>advanced.signature trans</Trans>);
    let content = (
        <div className="show-pri">
            <p><Trans>blockchain.transaction hash</Trans>: </p>
            <p>{data.result}</p>
        </div>
    );
    if (method === 'AppendSignature') {
        title = (<Trans>advanced.signature success</Trans>)
        content = (
            <div className="show-pri">
                <pre style={{ overflow: 'hidden', overflowX: 'auto', overflowY: 'scroll', maxHeight: '60vh', width: 'auto' }}>
                    <code>{JSON.stringify(JSON.parse(result), null, 2)}</code>
                </pre>
                <p>
                    <Button type="link" style={{ margin: 0, color: '#00B594' }} onClick={() => clickToCopy(result)}>
                        <Trans>button.copy to clipboard</Trans>
                    </Button>
                </p>
            </div>
        );
    }
    Modal.success({
        width: 650,
        centered: true,
        title: title,
        content: content,
        okText: <Trans>button.ok</Trans>
    });
}

function error(data) {
    let title = (<Trans>wallet.transfer send error</Trans>);
    let content = (
        <div className="show-pri">
            <p><Trans>blockchain.transaction hash</Trans>: {data.error.code}</p>
            <p><Trans>error msg</Trans>: {data.error.message}</p>
        </div>
    );

    if (data.error.code === 20014) {
        title = (<Trans>wallet.transfer send error 20014</Trans>);
        content = (
            <div className="show-pri">
                <pre style={{ overflow: 'hidden', overflowX: 'auto', overflowY: 'scroll', maxHeight: '60vh', width: 'auto' }}>
                    <code>{JSON.stringify(JSON.parse(data.error.message), null, 2)}</code>
                </pre>
                <p>
                    <Button type="link" style={{ margin: 0, color: '#00B594' }} onClick={() => clickToCopy(data.error.message)}>
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
    if (data.error.code === 20014) {
        Modal.warning(args);
    } else {
        Modal.error(args);
    }
}

@withTranslation()
@withAuthenticated
class Advancedsignature extends React.Component {
    constructor(props) {
        super(props);
        this.myForm = createRef();
        this.state = {
            visible: false,
            signres: ""
        };
    }
    onSign = values => {
        let params = { signContext: values.sign };
        var _this = this;
        post("AppendSignature", params).then(res => {
            var _data = res.data;
            if (_data.msgType === -1) {
                error(_data);
            } else if (_data.msgType === 3) {
                success(_data);
                // changeBroad(_data.result)
                _this.myForm.current.setFieldsValue({
                    broadcast: _data.result
                });
            }
        })
    };
    onBroad = values => {
        let params = { signContext: values.broadcast };
        post("BroadcastTransaction", params).then(res => {
            var _data = res.data;
            if (_data.msgType === -1) {
                error(_data);
            } else if (_data.msgType === 3) {
                success(_data);
            }
        })
    }
    render() {
        const { t } = this.props;
        return (
            <Layout className="gui-container">
                <Sync />
                <Content className="mt3">
                    <Row gutter={[30, 0]} className="mb1" style={{ 'minHeight': 'calc( 100vh - 150px )' }}>
                        <Col span={24} className="bg-white pv4">
                            <Tabs className="tran-title" defaultActiveKey="1"
                                items={[
                                    // {label:t("advanced.signature text"),key:1,children:(<div>文本签名</div>)},
                                    // {label:t("advanced.signature verify"),key:3,children:(<div>验证签名</div>)},
                                    {
                                        label: t("advanced.signature trans"), key: 2, children: (<div>
                                            <Form
                                                name="form"
                                                onFinish={this.onSign}
                                            >
                                                <h4>{t("advanced.trans")} Json</h4>
                                                <Form.Item
                                                    name="sign"
                                                    rules={[
                                                        {
                                                            required: true,
                                                            message: 'Please input your json!',
                                                        },
                                                    ]}
                                                >
                                                    <TextArea />
                                                </Form.Item>
                                                <Form.Item>
                                                    <Button type="primary" htmlType="submit">
                                                        {t('advanced.signature')}
                                                    </Button>
                                                </Form.Item>
                                            </Form>
                                            <Form
                                                name="form"
                                                onFinish={this.onBroad}
                                                ref={this.myForm}
                                            >
                                                <h4>{t('advanced.signature result')}</h4>
                                                <Form.Item
                                                    name="broadcast"
                                                    rules={[
                                                        {
                                                            required: true,
                                                            message: 'Please input your json!',
                                                        },
                                                    ]}
                                                >
                                                    <TextArea />
                                                </Form.Item>

                                                <Form.Item>
                                                    <Button type="primary" htmlType="submit">
                                                        {t('advanced.broadcast')}
                                                    </Button>
                                                </Form.Item>
                                            </Form>
                                        </div>)
                                    }
                                ]}
                            />
                        </Col>
                    </Row>
                </Content>
            </Layout>
        );
    }
}


// export { Advancedsignature }
export default Advancedsignature;


