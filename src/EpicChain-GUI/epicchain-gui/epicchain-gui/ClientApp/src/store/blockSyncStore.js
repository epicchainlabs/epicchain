/* eslint-disable */
import { observable, action, makeObservable } from "mobx";

class BlockSyncStore {
  constructor() {
    makeObservable(this);
  }

  @observable syncHeight = -1;
  @observable headerHeight = -1;

  @action setHeight(data) {
    this.syncHeight = data.syncHeight;
    this.headerHeight = data.headerHeight;
  }
}

export default BlockSyncStore;
