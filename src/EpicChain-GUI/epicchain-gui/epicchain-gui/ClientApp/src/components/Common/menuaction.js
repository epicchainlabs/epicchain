import React from "react";
import { useTranslation } from "react-i18next";
import { Form, message, Input, Button, Radio } from "antd";
import { walletStore } from "../../store/stores";
import { post } from "../../core/request";
import { shell } from "electron";
import { app, dialog } from '@electron/remote';
import { LockOutlined } from "@ant-design/icons";
import Config from "../../config";

const Addressdetail = () => {
  const { t } = useTranslation();
  const accounts = walletStore.accountlist;
  return (
    <div>
      <h4>{t("sideBar.address book")}</h4>
      <ul className="add-mark">
        {accounts.map((item, index) => {
          return (
            <li key={index}>
              {item.address}{" "}
              <span className="float-r mr2 small">
                - NEO <span>{item.neo}</span>
              </span>
            </li>
          );
        })}
      </ul>
      <div className="mt1 mb3 text-c small">
        <p className="mb5 t-light">
          NeoGUI @ 2024 The Neo Project/ {t("settings.copyright")}
        </p>
      </div>
    </div>
  );
};

const Changepass = ({ logout }) => {
  const { t } = useTranslation();
  const [form] = Form.useForm();
  const onFinish = (values) => {
    let params = {
      oldPassword: values.oldpass,
      newPassword: values.newpass,
    };
    console.log(params);
    post("ChangePassword", params)
      .then((res) => {
        var _data = res.data;
        if (_data.msgType === -1 || _data.result === false) {
          message.error(t("wallet.password change fail"));
          return;
        } else {
          message.success(t("wallet.password change success"));
          logout();
        }
      })
      .catch(function (error) {
        console.log("error");
        console.log(error);
      });
  };
  return (
    <div className="neo-form w300 mt3 mb2">
      <Form form={form} onFinish={onFinish}>
        <Form.Item
          name="oldpass"
          rules={[
            { required: true, message: t("wallet.please input password") },
          ]}
        >
          <Input.Password
            placeholder={t("wallet.please input old password")}
            maxLength={30}
            prefix={<LockOutlined />}
          />
        </Form.Item>
        <Form.Item
          name="newpass"
          rules={[
            { required: true, message: t("wallet.please input new password") },
          ]}
          hasFeedback
        >
          <Input.Password
            placeholder={t("wallet.please input new password")}
            maxLength={30}
            prefix={<LockOutlined />}
          />
        </Form.Item>
        <Form.Item
          name="veripass"
          dependencies={["newpass"]}
          hasFeedback
          rules={[
            {
              required: true,
              message: t("wallet.please confirm password"),
            },
            ({ getFieldValue }) => ({
              validator(rule, value) {
                if (!value || getFieldValue("newpass") === value)
                  return Promise.resolve();
                return Promise.reject(t("wallet.password not match"));
              },
            }),
          ]}
        >
          <Input.Password
            placeholder={t("wallet.please input twice")}
            maxLength={30}
            prefix={<LockOutlined />}
          />
        </Form.Item>
        <Form.Item>
          <Button type="primary" style={{ width: "100%" }} htmlType="submit">
            {t("button.confirm")}
          </Button>
        </Form.Item>
      </Form>
    </div>
  );
};

const Setting = ({ switchnetwork }) => {
  const { t, i18n } = useTranslation();
  const { Network } = Config;
  const switchLang = (lng) => {
    if (Config.Language === lng) {
      return;
    }
    i18n.changeLanguage(lng);
    Config.changeLang(lng);
  };
  const switchNetwork = (network) => {
    switchnetwork(network);
  };
  const openUrl = (url) => {
    return () => {
      shell.openExternal(url);
    };
  };
  return (
    <div>
      <h4>{t("settings.network")}</h4>

      <Radio.Group
        name="radiogroup"
        defaultValue={Network}
        onChange={(e) => switchNetwork(e.target.value)}
      >
        <Radio value="mainnet">{t("settings.mainnet")}</Radio>
        <Radio value="testnet">{t("settings.testnet")}</Radio>
        <Radio value="private">{t("settings.privatenet")}</Radio>
      </Radio.Group>
      <p className="small mt5">{t("settings.network info")}</p>

      <h4 className="mt3">{t("settings.language")}</h4>
      <Radio.Group
        className="setting-ul"
        defaultValue={i18n.language}
        onChange={(e) => switchLang(e.target.value)}
      >
        <Radio value="zh">中文</Radio>
        <Radio value="en">English</Radio>
      </Radio.Group>

      <h4 className="mt3">{t("settings.about")}</h4>
      <p className="font-s">
        {t("settings.version")} {app.getVersion()}
      </p>

      <div className="mt1 mb3 text-c small">
        <p className="mb5 t-light">
          NeoGUI @ 2024 Neo-Project {t("settings.copyright")}
        </p>
        <p>
          <a
            className="mr3 t-green"
            onClick={openUrl("https://github.com/neo-ngd/Neo3-GUI/issues")}
          >
            {t("settings.report issues")}
          </a>
          <a className="t-green" onClick={openUrl("https://neo.org/")}>
            Neo{t("settings.website")}
          </a>
        </p>
      </div>
    </div>
  );
};

export { Addressdetail, Changepass, Setting };
