/* eslint-disable */
Array.copy = function (src, srcOffset, dst, dstOffset, count) {
  for (var i = 0; i < count; i++) dst[i + dstOffset] = src[i + srcOffset];
};
Uint8Array.fromArrayBuffer = function (buffer) {
  if (buffer instanceof Uint8Array) return buffer;
  else if (buffer instanceof ArrayBuffer) return new Uint8Array(buffer);
  else {
    var view = buffer;
    return new Uint8Array(view.buffer, view.byteOffset, view.byteLength);
  }
};
String.prototype.hexToBytes = function () {
  if ((this.length & 1) !== 0) throw new RangeError();
  var bytes = new Uint8Array(this.length / 2);
  for (var i = 0; i < bytes.length; i++)
    bytes[i] = parseInt(this.substr(i * 2, 2), 16);
  return bytes;
};
Uint8Array.prototype.toHexString = function () {
  var s = "";
  for (var i = 0; i < this.length; i++) {
    s += (this[i] >>> 4).toString(16);
    s += (this[i] & 0xf).toString(16);
  }
  return s;
};
ArrayBuffer.prototype.toScriptHash = Uint8Array.prototype.toScriptHash = function () {
  return window.crypto.subtle
    .digest({ name: "SHA-256" }, this)
    .then(function (result) {
      return window.crypto.subtle.digest({ name: "RIPEMD-160" }, result);
    })
    .then(function (result) {
      return new Neo.Uint160(result);
    });
};
