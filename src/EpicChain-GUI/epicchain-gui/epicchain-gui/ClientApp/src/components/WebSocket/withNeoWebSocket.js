import React, { Component } from "react";
import neoWebSocket from "./neoWebSocket";

export const withNeoWebSocket = (ComposedCompnent) =>
  class extends Component {
    render() {
      return (
        <ComposedCompnent
          {...this.props}
          neoWs={neoWebSocket}
        ></ComposedCompnent>
      );
    }
  };
