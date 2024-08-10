import React from "react";
import { Input, PageHeader, Modal, Select, Row, Col, Form, message, Button, } from "antd";
import DataConverter from "./dataConverter";


class ParameterInput extends React.Component {
    constructor(props) {
        super(props);
        this.dataConverter = new DataConverter();
        //select type
        // let type = this.props.type;
        this.initComponentModel();
        console.log("init ParameterInput", this.props);
        this.state = {
            options: this.options || [],
        };
    }

    initComponentModel = () => {
        const { type } = this.props;
        switch (type) {
            case "ByteArray":
                this.options = ["HexString", "ByteArray", "String", "Integer"];
                // if (value) {
                //     this.hexValue = this.dataConverter.toBase64String(value);
                //     if (value.length == 40) {
                //         this.options.push("Hash160");
                //         this.options.push("Address");
                //     }
                //     if (value.length == 64) {
                //         this.options.push("Hash256");
                //     }
                // }
                break;
            case "Hash160":
                this.options = ["Hash160", "Address"];
                // if (value) {
                //     this.hexValue = value;
                // }
                break;
            case "String":
                this.options = ["ByteArray", "String"];
                // if (value) {
                //     this.hexValue = this.dataConverter.fromUtf8String(value);
                // }
                break;
            case "Integer":
                this.options = ["ByteArray", "Integer"];
                // if (value) {
                //     let hexInt = BigInt(value).toString(16);
                //     if (hexInt.startsWith("-")) {
                //         //unsupport negative number
                //         return;
                //     }
                //     if (hexInt.length & 1) {
                //         hexInt = "0" + hexInt;
                //     }
                //     this.hexValue = this.dataConverter.reverseHexString(hexInt);
                // }
                break;
            case "Any":
                this.options = ["HexString", "ByteArray", "String", "Integer", "Hash160", "Address", "Array"];
                break;
            default:
                this.options = [type];
                break;
        }
    };

    // setValue = (val) => {
    //     this.setState({ value: val });
    // };

    // onChangeType = (e) => {
    // const { type } = this.props;
    // const newType = e;
    // if (!this.hexValue) {
    //     return;
    // }
    // switch (newType) {
    //     case "Address":
    //         this.value = this.dataConverter.toAddress(this.hexValue);
    //         break;
    //     case "ByteArray":
    //         this.value = this.hexValue;
    //         break;
    //     case "Hash160":
    //     case "Hash256":
    //         this.value = "0x" + this.dataConverter.reverseHexString(this.hexValue);
    //         break;
    //     case "Base64":
    //         this.value = this.dataConverter.toBase64String(this.hexValue);
    //         break;
    //     case "String":
    //         this.value = this.dataConverter.toUtf8String(this.hexValue);
    //         break;
    //     case "Integer":
    //         this.value = this.dataConverter.toBigInt(this.hexValue).toString(10);
    //         break;
    // }
    // this.type = newType;
    // this.setValue(this.value);
    // }

    // onChangeValue = (e) => {
    //     let type = this.type;
    //     let value = e.currentTarget.value;
    //     if (!value) {
    //         this.hexValue = null;
    //         return;
    //     }
    //     switch (type) {
    //         case "ByteArray":
    //             this.hexValue = this.dataConverter.fromBase64String(value);
    //             break;
    //         case "Hash160":
    //         case "Hash256":
    //             this.hexValue = this.dataConverter.reverseHexString(value.replace('0x', ''));
    //             break;
    //         case "String":
    //             this.hexValue = this.dataConverter.fromUtf8String(value);
    //             break;
    //         case "Integer":
    //             this.hexValue = value.toString(16);
    //             break;
    //         case "Address":
    //             let scriptHash = this.dataConverter.toScriptHash(value);
    //             this.hexValue = this.dataConverter.reverseHexString(scriptHash.replace('0x', ''));
    //             break;
    //     }
    //     console.log(e);
    // }

    render() {
        const { name, type } = this.props;
        const { options } = this.state;
        return (
            <Form.Item className="spec-row" label={<span>{name}</span>} >
                <Input.Group compact>
                    <Form.Item
                        name={[name, "type"]}
                        style={{ display: "inline-block", width: "30%", }}
                        rules={[{ required: true, message: "type" }]}
                    >
                        <Select placeholder="Select Type" style={{ width: "100%" }} onChange={this.onChangeType} >
                            {
                                // key={[name, type, i]}
                                options.map((item, i) => (<Select.Option key={item}>{item}</Select.Option>))
                            }
                        </Select>
                    </Form.Item>
                    <Form.Item
                        name={[name, "value"]}
                        style={{ display: "inline-block", width: "70%", }}
                        rules={[{ required: type !== "Any", message: "value" },]}
                    >
                        <Input style={{ width: "100%" }} placeholder="Value" onChange={this.onChangeValue} />
                    </Form.Item>
                </Input.Group>
            </Form.Item>
        );
    };
}

export default ParameterInput;