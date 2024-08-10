import React from "react";
import "antd/dist/antd.min.css";
import { withTranslation } from "react-i18next";
import { Form, Input, Button, Select, Row, Col } from "antd";
import { PlusOutlined, MinusSquareOutlined } from "@ant-design/icons";

const { Option } = Select;

const formItemLayoutWithOutLabel = {
  wrapperCol: {
    sm: { span: 24, offset: 0 },
  },
};

const typeOption = [
  "Signature",
  "Boolean",
  "Integer",
  "Hash160",
  "Hash256",
  "ByteArray",
  "PublicKey",
  "String",
];

@withTranslation()
class DynamicArray extends React.Component {
  handleparam = (values) => {
    this.props.handleparam(values);
  };
  render() {
    const { t } = this.props;
    return (
      <Form
        name="dynamic_form"
        {...formItemLayoutWithOutLabel}
        onFinish={this.handleparam}
      >
        <Form.List name="guiarray">
          {(fields, { add, remove }) => {
            return (
              <div>
                {fields.map((field) => (
                  <Row key={field.key}>
                    <Col span="8">
                      <Form.Item
                        name={[field.name, "type"]}
                        label={t("Type")}
                        rules={[
                          {
                            required: true,
                            message: t("wallet.please select a account"),
                          },
                        ]}
                      >
                        <Select
                          placeholder={t("类型选择")}
                          style={{ width: "100%" }}
                        >
                          {typeOption.map((item) => {
                            return <Option key={item}>{item}</Option>;
                          })}
                        </Select>
                      </Form.Item>
                    </Col>
                    <Col span="16">
                      <Form.Item
                        name={[field.name, "amount"]}
                        label={t("参数")}
                        rules={[
                          {
                            required: true,
                            message: t("wallet.required"),
                          },
                        ]}
                      >
                        <Input placeholder="JSON" />
                      </Form.Item>
                    </Col>
                    {fields.length > 1 ? (
                      <Button
                        type="link"
                        className="delete-btn"
                        onClick={() => {
                          remove(field.name);
                        }}
                      >
                        <MinusSquareOutlined />{" "}
                        <span className="font-s">删除参数</span>
                      </Button>
                    ) : null}
                  </Row>
                ))}
                <Form.Item className="mt3 mb0">
                  <Button
                    type="dashed"
                    onClick={() => {
                      add();
                    }}
                  >
                    <PlusOutlined /> {t("添加参数")}
                  </Button>
                </Form.Item>
              </div>
            );
          }}
        </Form.List>

        <Form.Item>
          <Button type="primary" htmlType="submit">
            构造
          </Button>
        </Form.Item>
      </Form>
    );
  };
}

export default DynamicArray;
