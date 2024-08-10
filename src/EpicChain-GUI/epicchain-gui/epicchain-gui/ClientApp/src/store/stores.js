/* eslint-disable */
import BlockSyncStore from "./blockSyncStore";
import WalletStore from "./walletStore";
import NodeStore from "./nodeStore";

let blockSyncStore = new BlockSyncStore();
let walletStore = new WalletStore();
let nodeStore = new NodeStore();

const Stores = {
  nodeStore,
  walletStore,
  blockSyncStore,
};

export { blockSyncStore, walletStore, nodeStore };
export default Stores;
