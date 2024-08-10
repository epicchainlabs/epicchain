/* eslint-disable */
import React from 'react';
import 'antd/dist/antd.min.css';
import axios from 'axios';
import {
  Checkbox,
  PageHeader,
  Modal,
  Alert,
  Row,
  Col,
  Form,
  Select,
  Button,
  message
} from 'antd';
import { Layout } from 'antd';
import Sync from '../sync';
import { withTranslation } from "react-i18next";
import "../../static/css/advanced.css";
import { postAsync } from '../../core/request';

const { Option } = Select;
const { Content } = Layout;

@withTranslation()
class Advanceddesignrole extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      size: 'default',
      rolelist: [{ type: 4, role: "StateValidator" }, { type: 8, role: "Oracle" }],
    };
  }
  onRole = async (fieldsValue) => {
    const { t } = this.props;
    let response = await postAsync("GetNodesByRole", {
      "role": fieldsValue.role
    });
    console.log(response);
  }
  render() {
    const { t } = this.props;
    const { disabled, rolelist } = this.state;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row gutter={[30, 0]} style={{ 'minHeight': 'calc( 100vh - 120px )' }}>
            <Col span={24} className="bg-white pv4">
              <PageHeader title={t('指派节点角色')}></PageHeader>

              <div className="pa3">
                <Alert
                  className="mt3 mb3"
                  type="warning"
                  message="我也不知道该写什么，就先这么写吧"
                  showIcon
                />
                <Form ref="formRef" onFinish={this.onRole}>
                  <h4 className="bolder mb4">{t('advanced.be candidate')}</h4>
                  <Form.Item
                    name="节点角色类型"
                    className="select-role"
                    rules={[
                      {
                        required: true,
                        message: t("必须选择想要查询的节点"),
                      },
                    ]}
                  >
                    <Select
                      placeholder={t("advanced.select address")}
                      style={{ width: '100%' }}
                      onChange={this.setAddress}>
                      {rolelist.map((item) => {
                        return (
                          <Option key={item.role}>{item.name}</Option>
                        )
                      })}
                    </Select>
                  </Form.Item>
                  <p className="text-c mt3">
                    <Button type="primary" htmlType="submit" disabled={disabled} loading={this.state.iconLoading}>
                      {t("button.confirm")}
                    </Button>
                  </p>
                </Form>
              </div>
            </Col>
          </Row>
        </Content>
      </Layout>
    );
  }
}

export default Advanceddesignrole;