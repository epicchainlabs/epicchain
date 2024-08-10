import { spawn } from "child_process";
import path from "path";
import { app, dialog } from '@electron/remote';
import Config from "./config";

const isMac = process.platform === "darwin";
const isWin = process.platform === "win32";
const appPath = app.getAppPath();
// const isPack = remote.app.isPackaged;

class NeoNode {
  constructor() {
    this.pendingSwitchTimer = null;
  }

  debounce = (fn, wait) => {
    if (this.pendingSwitchTimer !== null) {
      clearTimeout(this.pendingSwitchTimer);
    }
    this.pendingSwitchTimer = setTimeout(fn, wait);
  };

  kill() {
    if (this.node) {
      this.node.kill();
      this.node = null;
    }
  }

  start(env, errorCallback) {
    if (isWin) {
      this.node = this.runCommand("./neo3-gui.exe", env, errorCallback);

    } else {
      this.node = this.runCommand("./neo3-gui", env, errorCallback);
    }
    // this.node = this.runCommand("dotnet neo3-gui.dll", env, errorCallback);
  }

  startNode(network, port, errorCallback) {
    const env = { NEO_NETWORK: network || "", NEO_GUI_PORT: port || "" };
    this.start(env, errorCallback);
  }

  /**
   * force restart node after 1 second (using config file)
   */
  switchNode(network) {
    console.log("switch to:", network);
    if (network) {
      Config.changeNetwork(network);
    }

    let retryCount = 0;
    this.delayStartNode(retryCount);
  }

  delayStartNode(retryCount) {
    retryCount = retryCount || 0;
    if (retryCount > 10) {
      console.log("stop retry");
      return;
    }
    if (this.node) {
      this.node.kill();
    }
    this.debounce(() => {
      this.startNode(Config.Network, Config.Port, () => {
        console.log(
          new Date(),
          retryCount + ":switch network fail:" + Config.Network
        );
        retryCount++;
        this.delayStartNode(retryCount);
      });
    }, 1000);
  }

  runCommand(command, env, errorCallback) {
    const startPath = appPath.replace("app.asar", "");
    console.log("startPath:", startPath);
    const parentEnv = process.env;
    const childEnv = { ...parentEnv, ...env };
    if (isWin) {
    } else if (isMac) {
      childEnv.PATH = childEnv.PATH + ":/usr/local/share/dotnet";
    } else {
    }

    const ps = spawn(command, [], {
      shell: false,
      encoding: "utf8",
      cwd: path.join(startPath, "build-neo-node"),
      env: childEnv,
    });
    ps.firstError = true;
    // ps.stdout.setEncoding('utf8');
    ps.stdout.on("data", (data) => {
      // var str = iconv.decode(new Buffer(data), 'gbk')
      // console.log(data.toString());
    });
    ps.stderr.setEncoding("utf8");
    ps.stderr.on("data", (data) => {
      // var str = iconv.decode(Buffer.from(data, 'binary'), 'cp936')
      // console.log("error str:", str);
      console.error(ps.pid + ":" + data.toString());
      if (ps.firstError && errorCallback) {
        ps.firstError = false;
        errorCallback(data.toString());
      }
    });
    ps.env = env;
    return ps;
  }
}

const singleton = new NeoNode();
export default singleton;
