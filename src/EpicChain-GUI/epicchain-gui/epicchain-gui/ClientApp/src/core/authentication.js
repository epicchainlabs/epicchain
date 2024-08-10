/* eslint-disable */
import React from "react";
import { observer, inject } from "mobx-react";
import Wallet from "../components/Wallet/wallet";
import { walletStore } from "../store/stores";

function Authenticated(Component) {
  // 组件有已登陆的模块 直接返回 (防止重新渲染)
  if (Component.AuthenticatedComponent) {
    return Component.AuthenticatedComponent;
  }

  // 创建验证组件
  @observer
  class AuthenticatedComponent extends React.Component {
    render() {
      const walletOpen = walletStore.isOpen;
      return (
        <div style={{ width: "100%", overflowY: "auto" }}>
          {walletOpen ? <Component {...this.props} /> : <Wallet />}
        </div>
      );
    }
  }

  Component.AuthenticatedComponent = AuthenticatedComponent;
  return Component.AuthenticatedComponent;
}


function withAuthenticated(Component) {
  function ComponentWithAuthenticatedProp(props) {
    const walletOpen = walletStore.isOpen;
    return (
      <div style={{ width: "100%", overflowY: "auto" }}>
        {walletOpen ? <Component {...props} /> : <Wallet />}
      </div>
    );
  }
  return observer(ComponentWithAuthenticatedProp);
}

export { Authenticated, withAuthenticated };
