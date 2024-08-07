---
title: Encode Algorithm
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




# Base58 and Base58Check

Within the EpicChain ecosystem, Base58 and Base58Check represent vital encoding and decoding schemes designed to facilitate seamless data exchange between binary (hexadecimal) and alphanumeric (ASCII) formats. This guide elaborates on the mechanisms of Base58 and Base58Check, explaining their interfaces, operational steps, and practical applications within EpicChain.

## Base58 Encoding

Base58 is preferred for data compression, readability, and creating encoding mechanisms resistant to automated monitoring. However, its lack of a verification check results in an inability to detect errors during transmission, leading to the development of Base58Check for enhanced reliability.

### Base58 Alphabet
EpicChain's Base58 alphabet includes numbers (1-9) and English letters, excluding O, I, and l to prevent confusion: `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`.

### Interface Definition
- **Encoding**: Converts byte array data into a Base58 string format (`string Encode(byte[] input)`).
- **Decoding**: Converts Base58 string format data back into a byte array (`byte[] Decode(string input)`).

### Encoding Steps
1. Prefix the byte array data with `0x00` and reverse its order (little endian).
2. Convert array data to a BigInteger object.
3. Convert BigInteger to a Base58-based number using the Base58 alphabet.
4. For each `0x00` in the original byte array, add the letter '1' at the start of the generated Base58 format data.

### Decoding Steps
1. Convert the Base58 input string into BigInteger format using the Base58 alphabet.
2. Convert BigInteger to a byte array and reverse the order (big endian).
3. If the byte array's first byte is `0x00` and the second byte is `>= 0x80`, start decoding from the second byte.
4. Count the number of the first letter in the Base58 alphabet and remove leading zeros from the decoded data.

## Base58Check Encoding

Base58Check builds upon Base58 by incorporating a checksum for data verification, enhancing the reliability of encoded data.

### Interface Definition
- **Encoding**: Encodes byte array data into a checkable Base58 string format (`string Base58CheckEncode(byte[] input)`).
- **Decoding**: Decodes checkable Base58 string data back into a byte array format (`byte[] Base58CheckDecode(string input)`).

### Encoding Steps
1. Double-SHA256 encode the input byte array and append the first 4 bytes of the hash as a checksum.
2. Encode the byte array, including the checksum, using Base58.

### Decoding Steps
1. Decode the Base58 input string to retrieve the byte array.
2. Verify the checksum by double-SHA256 encoding the data portion and comparing the first 4 bytes of the hash with the checksum.

## Practical Applications
- Import/export of secret keys in WIF format.
- Conversion between contract script hash and address.
- Import/export of secret keys in EEP2 format.

These encoding schemes play crucial roles in the security and functionality of the EpicChain network. By employing Base58Check, developers and users can ensure the integrity and correctness of critical data, such as secret keys and wallet addresses, promoting a safe and reliable blockchain environment.





<br/>