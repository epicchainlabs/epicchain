// const NodeManager = require('./node-manager');
const _process = process;
process.once("loaded", function () {
  global.process = _process;
  global.require = require;
  global.Buffer = Buffer;
});

// window.electron = Electron;
// window.ipcRenderer = require('electron').ipcRenderer;
// window.remote = require('electron').remote;
// window.nodeManager = new NodeManager();
