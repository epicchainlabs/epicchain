/* eslint-disable */
import React from "react";
import "antd/dist/antd.min.css";
import "../../static/css/trans.css";
import "../../static/js/bundledemo.js";
import {
  Divider,
  Input,
  Tooltip,
  Icon,
  Cascader,
  Modal,
  Drawer,
  message,
  Button,
  AutoComplete,
} from "antd";
import "../../static/css/wallet.css";
import DataConvert from "./dataConverter";
import { withTranslation } from "react-i18next";

import { SwapOutlined } from "@ant-design/icons";

@withTranslation()
class Datatrans extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      outstr: "",
      outhexstr: "",
      outhash: "",
      outbighash: "",
      outbigadd: "",
      outlittlehash: "",
      outlittleadd: "",
      outhexnum: "",
      outlittle64: "",
    };
  }
  convert = new DataConvert();
  componentDidMount() { }
  hexToString = (hex) => {
    var trimhex = hex.trim();
    var rawStr =
      trimhex.substr(0, 2).toLowerCase() === "0x" ? trimhex.substr(2) : trimhex;
    var len = rawStr.length;
    if (len % 2 !== 0) {
      message.error("Illegal Format ASCII Code!");
      return "";
    }
    var cuChar;
    var result = [];
    for (var i = 0; i < len; i = i + 2) {
      cuChar = parseInt(rawStr.substr(i, 2), 16);
      result.push(String.fromCharCode(cuChar));
    }
    return result;
  };
  stringToHex = (str) => {
    if (str === "") return "";
    var hexChar = [];
    for (var i = 0; i < str.length; i++) {
      hexChar.push(str.charCodeAt(i).toString(16));
    }
    return hexChar;
  };
  stringTrans = () => {
    let instr = document.getElementById("inString").value;
    if (instr) {
      var _outStr = this.hexToString(instr);
      this.setState({
        outstr: _outStr,
      });
    }
    let inhexstr = document.getElementById("inHexString").value;
    if (inhexstr) {
      var _outhexstr = this.stringToHex(inhexstr).join("");
      this.setState({
        outhexstr: _outhexstr,
      });
    }
  };
  endianTrans = () => {
    const { t } = this.props;
    let inhash = document
      .getElementById("inHash")
      .value.replace(/(^\s*)|(\s*$)/g, "");
    if (inhash.length !== 40 && inhash.length !== 42) {
      message.error(t("datatrans.input error"));
      return;
    }

    var result = [],
      num;
    if (inhash.indexOf("0x") == 0) {
      inhash = inhash.slice(2);
    } else if (inhash) {
      result = ["0x"];
    }

    var hashArray = inhash.hexToBytes().reverse();
    for (var i = 0; i < hashArray.length; i++) {
      num = hashArray[i];
      if (num < 16) {
        num = hashArray[i].toString(16);
        num = "0" + num;
      } else {
        num = hashArray[i].toString(16);
      }
      result.push(num);
    }
    let _outhash = result.join("");
    this.setState({
      outhash: _outhash,
    });
  };
  littleTrans = () => {
    const { t } = this.props;
    var _this = this;
    var inlittlehash = document
      .getElementById("inLittleHash")
      .value.replace(/(^\s*)|(\s*$)/g, "");
    if (inlittlehash) {
      if (inlittlehash.substr(0, 2) == "0x")
        inlittlehash = inlittlehash.slice(2);
      if (inlittlehash.length != 40) {
        message.error(t("datatrans.input error"));
        return;
      }
      let _address = this.convert.toAddress(inlittlehash);
      _this.setState({
        outlittlehash: _address,
      });
    }
    var inlittleadd = document
      .getElementById("inLittleAddress")
      .value.replace(/(^\s*)|(\s*$)/g, "");
    if (inlittleadd) {
      if (inlittleadd.length != 34) {
        message.error(t("datatrans.input error"));
        return;
      }
      let _hash = this.convert.toScriptHash(inlittleadd);
      _hash = _hash.replace(/0x/g, "");
      _this.setState({
        outlittleadd: _hash,
      });
      let _base64 = this.convert.toBase64String(_hash);
      _this.setState({
        outlittle64: _base64,
      });
    }
  };
  bigTrans = () => {
    const { t } = this.props;
    var _this = this;
    var inbighash = document
      .getElementById("inBigHash")
      .value.replace(/(^\s*)|(\s*$)/g, "");
    if (inbighash) {
      if (inbighash.substr(0, 2) == "0x") inbighash = inbighash.slice(2);
      if (inbighash.length != 40) {
        message.error(t("datatrans.input error"));
        return;
      }
      let _little = this.convert.reverseHexString(inbighash);
      let _address = this.convert.toAddress(_little);
      _this.setState({
        outbighash: _address,
      });
    }
    var inbigadd = document
      .getElementById("inBigAdd")
      .value.replace(/(^\s*)|(\s*$)/g, "");
    if (inbigadd) {
      if (inbigadd.length != 34) {
        message.error(t("datatrans.input error"));
        return;
      }
      let _hash = this.convert.toScriptHash(inbigadd);
      _hash = this.convert.reverseHexString(_hash).replace(/0x/g, "");
      _hash = "0x" + _hash;
      _this.setState({
        outbigadd: _hash,
      });
    }
  };
  base64Trans = () => {
    const { t } = this.props;
    var _this = this;
    var inhexhash = document
      .getElementById("inHexHash")
      .value.replace(/(^\s*)|(\s*$)/g, "");
    if (inhexhash) {
      let _base64 = this.convert.toBase64String(inhexhash);
      _this.setState({
        outhexhash: _base64,
      });
    }
    var inbasehash = document
      .getElementById("inBaseHash")
      .value.replace(/(^\s*)|(\s*$)/g, "");
    if (inbasehash) {
      // let _hash = this.convert.toHexString(inbasehash);
      let _hash = Buffer.from(inbasehash, "base64").toString("hex");
      _this.setState({
        outbasehash: _hash,
      });
    }
  };
  // numTrans = () =>{
  //     var _this = this;
  //     var inhexnum = document.getElementById("inHexNum").value.replace(/(^\s*)|(\s*$)/g, "");
  //     if (inhexnum) {
  //         // if(inhexnum.substr(0, 2)=="0x")inhexnum = inhexnum.slice(2);
  //         // if(inhexnum.length!=40){message.error("Hash (Little)的格式错误，请检查后再试");return;}
  //         let _num = this.convert.toNumber(inhexnum);
  //         console.log(_num)
  //         if(!_num) {message.error("Hash (Little)的格式错误，请检查后再试");return;}
  //         _this.setState({
  //             outhexnum: _num
  //         })
  //     }
  //     var inlittleadd = document.getElementById("inLittleAdd").value.replace(/(^\s*)|(\s*$)/g, "");
  //     if (inlittleadd) {
  //         if(inlittleadd.length!=34){message.error("Address 的格式错误，请检查后再试");return;}
  //         let _hash = this.convert.toScriptHash(inlittleadd);
  //         _hash = this.convert.reverseHexString(_hash);
  //         _this.setState({
  //             outlittleadd: _hash
  //         },() => {this.state})
  //     }
  // }
  render() {
    return (
      <Drawer
        title="Data Transform"
        width={500}
        placement="right"
        closable={false}
        onClose={this.props.onClose}
        open={this.props.visible}
        getContainer={false}
        style={{ position: "absolute" }}
      >
        <ul className="trans-ul datatrans">
          <li>
            <Divider className="font-n">
              String <SwapOutlined className="small" /> Hex String
            </Divider>
            <p className="trans-area">
              <label>Hex String:</label>
              <Input id="inString" type="text" placeholder="7472616e73666572" />
              <label>String:</label>
              <span id="outString" className="trans-text">
                {this.state.outstr}
              </span>
              <br />
            </p>
            <p className="trans-area">
              <label>String:</label>
              <Input id="inHexString" type="text" placeholder="transfer" />
              <label>Hex String:</label>
              <span id="outHexString" className="trans-text">
                {this.state.outhexstr}
              </span>
              <br />
            </p>
            <p className="text-r">
              <Button type="primary" onClick={this.stringTrans}>
                Transform
              </Button>
            </p>
          </li>
          <li>
            <Divider className="font-n">
              Hex string <SwapOutlined className="small" /> Base 64
            </Divider>
            <p className="trans-area">
              <label>Hex String:</label>
              <Input id="inHexHash" type="text" placeholder="63a3989cb4a99571aa00396d3c7155faea4098ba" />
              <label>Base64:</label>
              <span id="outString" className="trans-text">
                {this.state.outhexhash}
              </span>
              <br />
            </p>
            <p className="trans-area">
              <label>Base64:</label>
              <Input id="inBaseHash" type="text" placeholder="Y6OYnLSplXGqADltPHFV+upAmLo=" />
              <label>Hex String:</label>
              <span id="outHexString" className="trans-text">
                {this.state.outbasehash}
              </span>
              <br />
            </p>
            <p className="text-r">
              <Button type="primary" onClick={this.base64Trans}>
                Transform
              </Button>
            </p>
          </li>
          <li>
            <Divider className="font-n mt3">
              Address <SwapOutlined className="small" /> Script Hash ( little
              endian )
            </Divider>
            <p className="trans-area">
              <label>Hash:</label>
              <Input
                id="inLittleHash"
                type="text"
                placeholder="b135cda6d0c707b8fd019cb76f555eb518f94945"
              />
              <label>Address:</label>
              <span className="trans-text">{this.state.outlittlehash}</span>
              <br />
            </p>
            <p className="trans-area">
              <label>Address:</label>
              <Input
                id="inLittleAddress"
                type="text"
                placeholder="Nc4yF2jDZkhrm2EnkRe8KjY6CRkATfn7hm"
              />
              <label>Hash:</label>
              <span className="trans-text">{this.state.outlittleadd}</span>
              <br />
              <label>Base 64:</label>
              <span className="trans-text">{this.state.outlittle64}</span>
              <br />
            </p>
            <p className="text-r">
              <Button type="primary" onClick={this.littleTrans}>
                Transform
              </Button>
            </p>
          </li>
          <li>
            <Divider className="font-n">
              Address <SwapOutlined className="small" /> Hex String ( big endian
              )
            </Divider>
            <p className="trans-area">
              <label>Hash:</label>
              <Input
                id="inBigHash"
                type="text"
                placeholder="0x4549f918b55e556fb79c01fdb807c7d0a6cd35b1"
              />
              <label>Address:</label>
              <span className="trans-text">{this.state.outbighash}</span>
              <br />
            </p>
            <p className="trans-area">
              <label>Address:</label>
              <Input
                id="inBigAdd"
                type="text"
                placeholder="Nc4yF2jDZkhrm2EnkRe8KjY6CRkATfn7hm"
              />
              <label>Hash:</label>
              <span className="trans-text">{this.state.outbigadd}</span>
              <br />
            </p>
            <p className="text-r">
              <Button type="primary" onClick={this.bigTrans}>
                Transform
              </Button>
            </p>
          </li>
          <li>
            <Divider className="font-n">
              Script Hash (Big-endian <SwapOutlined className="small" />{" "}
              Little-endian)
            </Divider>
            <p className="trans-area">
              <label>Big / Little:</label>
              <Input
                id="inHash"
                type="text"
                placeholder="0xb135cda6d0c707b8fd019cb76f555eb518f94945"
              />
              <label>Result:</label>
              <span className="trans-text">{this.state.outhash}</span>
              <br />
            </p>
            <p className="text-r">
              <Button type="primary" onClick={this.endianTrans}>
                Transform
              </Button>
            </p>
          </li>
        </ul>
      </Drawer>
    );
  }
}

export default Datatrans;
