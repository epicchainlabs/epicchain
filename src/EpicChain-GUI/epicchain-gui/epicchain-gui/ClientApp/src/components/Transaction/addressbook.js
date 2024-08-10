/* eslint-disable */
import React from "react";
import "antd/dist/antd.min.css";
import axios from "axios";
import { Table, Divider, Tag } from "antd";

let request, db;

class AddressBook extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      dataSource: [],
    };
  }
  componentDidMount() {
    request = window.indexedDB.open("demo", 2);
    request.onsuccess = function (event) {
      db = request.result;
      request.onupgradeneeded = function (event) {
        db = event.target.result;
        var objectStore = db.createObjectStore("person", { keyPath: "id" });
        objectStore.createIndex("name", "name", { unique: false });
        objectStore.createIndex("email", "email", { unique: true });
      };
    };
    request.onerror = function (event) {
      console.log("数据库打开报错");
    };
  }
  getAddress = () => { };
  addAddress = () => { };
  delAddress = () => { };
  updateAddress = () => { };
  deTable = () => { };
  render() {
    return (
      <div>
        <h1>地址簿</h1>
        <button onClick={this.deTable}>删除表结构</button>
        <Table dataSource={dataSource} columns={columns} />
      </div>
    );
  }
}

const dataSource = [
  {
    key: "1",
    name: "胡彦斌",
    age: 32,
    address: "西湖区湖底公园1号",
  },
  {
    key: "2",
    name: "胡彦祖",
    age: 42,
    address: "西湖区湖底公园1号",
  },
];

const columns = [
  {
    title: "地址",
    dataIndex: "address",
    key: "address",
  },
  {
    title: "备注",
    dataIndex: "note",
    key: "note",
  },
  {
    title: "操作",
    dataIndex: "age",
    key: "age",
  },
];

export default AddressBook;
