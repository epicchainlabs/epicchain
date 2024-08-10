import React, { useEffect } from "react";
import Router from "./router/router";
import { ConfigProvider } from "antd";
import { Provider } from "mobx-react";
import stores from "./store/stores";
import neoNode from "./neonode";
import neoWebSocket from "./components/WebSocket/neoWebSocket";


const processGetSyncHeight = (msg) => {
  stores.blockSyncStore.setHeight(msg.result);
}

const processGetWalletBalance = (msg) => {
  stores.walletStore.setAccounts(msg.result.accounts);
  stores.walletStore.setUnclaimedGas(msg.result.unclaimedGas);
}

class App extends React.Component {
  constructor(props) {
    super(props);

    console.log(window.location.href);
    if (process.env.NODE_ENV !== "development") {
      neoNode.switchNode();
    }

    neoWebSocket.initWebSocket();
    neoWebSocket.registMethod("getSyncHeight", processGetSyncHeight);
    neoWebSocket.registMethod("getWalletBalance", processGetWalletBalance);
  }

  componentWillUnmount = () => {
    neoWebSocket.unregistMethod("getSyncHeight", processGetSyncHeight);
    neoWebSocket.unregistMethod("getWalletBalance", processGetWalletBalance);
  };

  render() {
    return (
      <Provider {...stores}>
        <ConfigProvider>
          <Router></Router>
        </ConfigProvider>
      </Provider>
    );
  }
}

// function App() {
//   console.log(window.location.href);
//   if (process.env.NODE_ENV !== "development") {
//     neoNode.switchNode();
//   }

//   neoWebSocket.initWebSocket();
//   neoWebSocket.registMethod("getSyncHeight", processGetSyncHeight);
//   neoWebSocket.registMethod("getWalletBalance", processGetWalletBalance);


//   componentWillUnmount = () => {
//     neoWebSocket.unregistMethod("getSyncHeight", processGetSyncHeight);
//     neoWebSocket.unregistMethod("getWalletBalance", processGetWalletBalance);
//   };
//   return (
//     <Provider {...stores}>
//       <ConfigProvider>
//         <Router></Router>
//       </ConfigProvider>
//     </Provider>
//   );
// }





export default App;
