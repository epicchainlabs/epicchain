/* eslint-disable */
import React from 'react';
import { Link } from 'react-router-dom';
import 'antd/dist/antd.min.css';
import '../../static/css/menu.css'
import '../../static/css/contract.css'
import { Layout, List, Row, Col, PageHeader, Typography, Avatar } from 'antd';
import Sync from '../sync';
import Searcharea from './searcharea'
import { withTranslation } from "react-i18next";
import { Copy } from '../copy';
import { postAsync } from '../../core/request';

const { Content } = Layout;

@withTranslation()
class Contract extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      size: 'default',
      visible: false,
      show: false,
      assetlist: []
    };
  }
  componentDidMount() {
    this.getAllContracts();
  }
  visi = () => {
    this.setState({
      show: !this.state.show,
    });
  }
  getAllContracts = async (info) => {
    let response = await postAsync("GetAllContracts");
    if (response.msgType === -1) {
      message.error("查询失败");
      return;
    }
    this.setState({
      assetlist: response.result
    })
  }
  render() {
    const { t } = this.props;
    const { assetlist } = this.state;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row gutter={[30, 0]} style={{ 'minHeight': 'calc( 100vh - 135px )' }}>
            <Col span={24} className="bg-white pv4">
              <PageHeader title={t("contract.search contract")}></PageHeader>
              <List
                itemLayout="horizontal"
                dataSource={assetlist}
                className="font-s"
                renderItem={item => (
                  <List.Item>
                    <List.Item.Meta
                      avatar={
                        <Avatar src={"https://neo.org/images/gui/" + item.hash + ".png"} />
                      }
                      title={<Link className="asset-link w450 ellipsis" to={"/contract/detail:" + item.hash} title={t("show detail")}>{item.name}</Link>}
                    />
                    <Typography className='code'>{item.hash} <Copy msg={item.hash} /></Typography>
                  </List.Item>
                )}
              />
            </Col>

            <Searcharea />
          </Row>
        </Content>
      </Layout>
    );
  }
}

export default Contract;