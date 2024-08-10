import React from "react";
import DataConverter from "./dataConverter";

class ParameterValue extends React.Component {
  constructor(props) {
    super(props);
    this.dataConverter = new DataConverter();
    //show value
    this.value = this.props.value;
    //select type
    this.type = this.props.type;
    // value's Hex format string
    this.hexValue = null;
    //convert value to hex value and set options
    this.initComponentModel();
    this.state = {
      value: this.value,
      type: this.type,
      options: this.options || [],
    };
  }

  initComponentModel = () => {
    const { type, value } = this.props;
    switch (type) {
      case "ByteArray":
        this.hexValue = value;
        this.options = ["ByteArray", "String", "Integer", "Base64"];
        if (value.length == 40) {
          this.options.push("Hash160");
          this.options.push("Address");
        }
        if (value.length == 64) {
          this.options.push("Hash256");
        }
        break;
      case "String":
        this.hexValue = this.dataConverter.fromUtf8String(value);
        this.options = ["ByteArray", "String", "Base64"];
        break;
      case "Integer":
        let hexInt = BigInt(value).toString(16);
        if (hexInt.startsWith("-")) {
          //unsupport negative number
          return;
        }
        if (hexInt.length & 1) {
          hexInt = "0" + hexInt;
        }
        this.hexValue = this.dataConverter.reverseHexString(hexInt);
        this.options = ["ByteArray", "Integer"];
        break;
      default:
        this.options = [this.type];
        break;
    }
  };

  onChangeType = (e) => {
    const newType = e.target.value;
    switch (newType) {
      case "Address":
        this.value = this.dataConverter.toAddress(this.hexValue);
        break;
      case "ByteArray":
        this.value = this.hexValue;
        break;
      case "Hash160":
      case "Hash256":
        this.value = "0x" + this.dataConverter.reverseHexString(this.hexValue);
        break;
      case "Base64":
        this.value = this.dataConverter.toBase64String(this.hexValue);
        break;
      case "String":
        this.value = this.dataConverter.toUtf8String(this.hexValue);
        break;
      case "Integer":
        this.value = this.dataConverter.toBigInt(this.hexValue).toString(10);
        break;
    }

    this.setState({
      type: newType,
      value: this.value,
    });
  };

  componentDidMount = () => { };

  render() {
    const { value, type, options } = this.state;
    const optionItems = options.map((item, i) => (
      <option key={i}>{item}</option>
    ));
    return (
      <div>
        <select value={type} onChange={this.onChangeType}>
          {optionItems}
        </select>
        <label>{value}</label>
      </div>
    );
  };
}

export default ParameterValue;
