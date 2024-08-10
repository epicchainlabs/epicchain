/* eslint-disable */
import bs58check from "bs58check";
import { NEO3_ADDRESS_VERSION } from "../../constants";

class DataConverter {
  /**
   * convert scriptHash to address
   *  @param {string} scriptHashHex
   */
  toAddress(scriptHashHex) {
    let littleEndian = scriptHashHex;
    if (scriptHashHex.startsWith("0x")) {
      littleEndian = this.reverseHexString(scriptHashHex.slice(2));
    }
    const bytes = Buffer.from(NEO3_ADDRESS_VERSION + littleEndian, "hex");
    if (bytes.length != 21) {
      return null;
    }
    const address = bs58check.encode(bytes);
    return address;
  }

  /**
   * convert address to scriptHash
   * @param {string} address
   */
  toScriptHash(address) {
    var bytes = bs58check.decode(address);
    return "0x" + Buffer.from(bytes).toString("hex").slice(2);
  }
  
  /**
   * direct reverse hexstring,input:'f9df308b7bb380469354062f6b73f9cb0124317b',output:'7b312401cbf9736b2f0654934680b37b8b30dff9'
   * @param {string} hexString
   */
  reverseHexString(hexString) {
    if (hexString.length & 1) {
      throw new RangeError(hexString);
    }
    return hexString.match(/../g).reverse().join("");
  }

  /**
   * input:'63a3989cb4a99571aa00396d3c7155faea4098ba',output:'Y6OYnLSplXGqADltPHFV+upAmLo='
   * @param {*} hexString
   */
  toBase64String(hexString) {
    return Buffer.from(hexString, "hex").toString("base64");
  }

  /**
   * input:'Y6OYnLSplXGqADltPHFV+upAmLo=',output:'63a3989cb4a99571aa00396d3c7155faea4098ba'
   * @param {*} base64
   */
  toHexString(str) {
    return Buffer.from(str, "base64").toString("hex");
  }

  /**
   * Encode hex string to utf8 string,input:'7472616e73666572',output:'transfer'
   * @param {*} hexString
   */
  toUtf8String(hexString) {
    return Buffer.from(hexString, "hex").toString("utf8");
  }

  /**
   * Decode utf8 string to hex string,input:'transfer',output:'7472616e73666572'
   * @param {*} text
   */
  fromUtf8String(text) {
    return Buffer.from(text, "utf8").toString("hex");
  }

  /**
   * Hex string to BigInt
   * @param {*} hexString
   */
  toBigInt(hexString) {
    if (!hexString.startsWith("0x")) {
      hexString = "0x" + this.reverseHexString(hexString);
    }
    return BigInt(hexString);
  }
}

export default DataConverter;
