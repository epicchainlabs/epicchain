/* eslint-disable */
import { observable, action, makeObservable } from "mobx";

class WalletStore {
  constructor() {
    makeObservable(this);
  }

  @observable isOpen = false;
  @observable accountlist = [];
  @observable unclaimedGas = "";

  @action logout() {
    this.isOpen = false;
    this.accountlist = [];
  }

  @action setWalletState(isopen) {
    console.log("set wallet:", isopen);
    this.isOpen = isopen;
  }

  @action setAccounts(accounts) {
    this.accountlist = accounts;
    // if (accounts && !this.isOpen) {
    //   this.setWalletState(true);
    // }
  }

  @action addAccount(account) {
    this.accountlist = this.accountlist.concat(account);
  }

  @action setUnclaimedGas(gas) {
    this.unclaimedGas = gas;
  }
}

export default WalletStore;
