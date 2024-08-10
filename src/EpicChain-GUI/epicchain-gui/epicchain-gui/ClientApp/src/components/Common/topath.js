import React from "react";
import "antd/dist/antd.min.css";
import { Navigate } from "react-router-dom";

class Topath extends React.Component {
  render() {
    if (!this.props.topath) return <div></div>;
    return <Navigate to={this.props.topath} />;
  }
}

export default Topath;
