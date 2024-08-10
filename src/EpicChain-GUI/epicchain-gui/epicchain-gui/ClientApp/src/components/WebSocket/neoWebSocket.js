import Config from "../../config";
class NeoWebSocket {
  constructor() {
    this.processMethods = {};
    this.lock = false;
    this.ws = null;
  }

  log() {
    console.log("NeoWebSocket=>", ...arguments);
  }

  warn() {
    console.warn("NeoWebSocket=>", ...arguments)
  }


  initWebSocket = () => {
    try {
      this.ws = this.createWebSocket();
    } catch (error) {
      this.warn(error);
    }
  };

  createWebSocket = () => {
    this.log("creating new webscoket");
    let ws = new WebSocket(Config.getWsUrl());

    ws.onopen = () => {
      this.log("[opened]");
    };

    ws.onclose = (e) => {
      this.warn("[closed]", e);
      this.reconnectWebSocket();
    };

    ws.onerror = (e) => {
      this.warn("[error]", e);
    };

    ws.onmessage = this.processMessage;
    return ws;
  };

  reconnectWebSocket = () => {
    let self = this;
    if (this.lock) {
      return;
    }
    this.lock = true;
    setTimeout(() => {
      this.initWebSocket();
      this.lock = false;
    }, 5000);
  };

  /**
   * distribute websocket message to regitered process methods
   */
  processMessage = (message) => {
    let msg = JSON.parse(message.data);
    let methods = this.processMethods[msg.method];
    if (methods && methods.length > 0) {
      for (let methodFunc of methods) {
        try {
          methodFunc(msg);
        } catch (error) {
          this.log(error);
        }
      }
    }
  };

  /**
   * regiter new process method
   * @param {*} method message method name
   * @param {*} func process method
   */
  registMethod(method, func) {
    let methods = this.processMethods[method];
    if (!(methods && methods.length)) {
      methods = [];
    }
    methods.push(func);
    this.processMethods[method] = methods;
  }

  /**
   * unregiter process method
   * @param {*} method
   * @param {*} func
   */
  unregistMethod(method, func) {
    let methods = this.processMethods[method];
    if (methods && methods.length) {
      let i = 0;
      while (i < methods.length) {
        if (methods[i] === func) {
          methods.splice(i, 1);
        } else {
          ++i;
        }
      }
      this.processMethods[method] = methods;
    }
  }
}

const instance = new NeoWebSocket();
export default instance;
