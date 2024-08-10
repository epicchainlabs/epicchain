/* eslint-disable */
import React from 'react';
import '../../static/css/trans.css';
import { Layout, Row, Col, Tabs, message } from 'antd';
import Translog, { Hashdetail, Attrlist, Translist, Witlist } from './translog';
import Datatrans from '../Common/datatrans';
import Sync from '../sync';
import { SwapOutlined, ArrowLeftOutlined } from '@ant-design/icons';
import { useTranslation, withTranslation } from "react-i18next";
import { post } from "../../core/request";
import withRouter from '../../core/withRouter';

const { Content } = Layout;
@withTranslation()
@withRouter
class Untransdetail extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      visible: false,
      hashdetail: [],
      transfers: [],
      witnesses: [],
      attributes: [],
      notifies: []
    };
  }
  componentDidMount() {
    this.getTransdetail(res => {
      console.log(res);
      this.setState({
        hashdetail: res,
        transfers: res.transfers,
        witnesses: res.witnesses,
        attributes: res.attributes,
        notifies: res.notifies,
      });
    });
  }
  showDrawer = () => {
    this.setState({
      visible: true,
    });
  };
  onClose = () => {
    this.setState({
      visible: false,
    });
  };
  getTransdetail = callback => {
    var _this = this;
    let params = {
      "txId": location.pathname.split(":").pop()
    };
    post("GetUnconfirmedTransaction", params).then(res => {
      var _data = res.data;
      console.log(_data)
      if (_data.msgType === -1) {
        message.error("查询失败");
        return;
      } else {
        callback(_data.result);
      }
    }).catch(function () {
      console.log("error");
      _this.props.router.navigate(-1);
    });
  }
  back = () => {
    const { location, navigate } = this.props.router;
    const from = location.state?.from || null;
    if (from) {
      navigate(from, { replace: true });
    } else {
      navigate(-1);
    }
  }
  render() {
    const { t } = this.props;
    const { hashdetail, transfers, witnesses, attributes } = this.state;
    return (
      <Layout className="gui-container">
        <Sync />
        <Content className="mt3">
          <Row gutter={[30, 0]} className="mb1">
            <Col span={24} className="bg-white pv4">
              <a className="fix-btn" onClick={this.showDrawer}><SwapOutlined /></a>
              <Tabs className="tran-title" defaultActiveKey="1" tabBarExtraContent={<ArrowLeftOutlined className="h2" onClick={this.back} />}
                items={[
                  {
                    label: t("blockchain.transaction.content"), key: 1, children: (
                      <div>
                        <Hashdetail hashdetail={hashdetail} />
                        <Translist transfers={transfers} />
                        <Attrlist attributes={attributes} />
                        <Witlist witnesses={witnesses} />
                      </div>)
                  }
                ]}>

              </Tabs>
            </Col>
          </Row>
          <Datatrans visible={this.state.visible} onClose={this.onClose} />
        </Content>
      </Layout>
    );
  }
}

export default Untransdetail;