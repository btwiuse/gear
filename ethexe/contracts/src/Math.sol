// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.26;

import {IMath} from "./IMath.sol";
import {IMirror} from "./IMirror.sol";
import {ScaleCodec} from "./ScaleCodec.sol";

contract Math is IMath {
    modifier onlyMirror() {
        require(msg.sender == mirror, "not mirror contract");
        _;
    }

    address public mirror;

    function initialize(address _mirror) external {
        require(mirror == address(0), "Already initialized");
        mirror = _mirror;
    }

    function emitMathMinReply(bytes memory _payload, address _destination, uint128 _value, bytes32 _replyTo, bytes4 _replyCode, uint offset) private {
        uint256 payload = ScaleCodec.decodeUint256(_payload, offset);
        offset += 32;
        emit MathMinReply(payload, _destination, _value, _replyTo, _replyCode);
    }

    function emitMathSqrtReply(bytes memory _payload, address _destination, uint128 _value, bytes32 _replyTo, bytes4 _replyCode, uint offset) private {
        uint256 payload = ScaleCodec.decodeUint256(_payload, offset);
        offset += 32;
        emit MathSqrtReply(payload, _destination, _value, _replyTo, _replyCode);
    }

    function onMessageSent(bytes32 id, address destination, bytes calldata payload, uint128 value) public onlyMirror {
        emit OnMessageEvent(payload, id, destination, value);
    }

    function onReplySent(address destination, bytes calldata payload, uint128 value, bytes32 replyTo, bytes4 replyCode) public onlyMirror {
        if (replyCode[0] != 0x00) {
            emit ErrorReply(payload, destination, value, replyTo, replyCode);
            return;
        }
        uint256 offset = 0;
        if (ScaleCodec.isBytesPrefixedWith(hex"104d617468", payload, offset)) {
            offset = 5;
            if (uint8(payload[offset]) == 0x0c) {
                emitMathMinReply(payload, destination, value, replyTo, replyCode, offset + 4);
            }
            else if (uint8(payload[offset]) == 0x10) {
                emitMathSqrtReply(payload, destination, value, replyTo, replyCode, offset + 5);
            }
        }
        else {
            emit OnReplyEvent(payload, destination, value, replyTo, replyCode);
        }
    }

    function fnMathMin(uint256 x, uint256 y, uint128 _value) external payable {
        uint256 payloadLen = 9;
        payloadLen += 32;
        payloadLen += 32;
        bytes memory payload = new bytes(payloadLen);
        ScaleCodec.insertBytesTo(hex"104d6174680c4d696e", payload, 0);
        uint256 offset = 9;
        ScaleCodec.encodeUint256To(x, payload, offset);
        offset += 32;
        ScaleCodec.encodeUint256To(y, payload, offset);
        offset += 32;
        sendMessage(payload, _value);
    }

    function fnMathSqrt(uint256 y, uint128 _value) external payable {
        uint256 payloadLen = 10;
        payloadLen += 32;
        bytes memory payload = new bytes(payloadLen);
        ScaleCodec.insertBytesTo(hex"104d6174681053717274", payload, 0);
        uint256 offset = 10;
        ScaleCodec.encodeUint256To(y, payload, offset);
        offset += 32;
        sendMessage(payload, _value);
    }

    function sendMessage(bytes memory _payload, uint128 _value) public payable {
        IMirror(mirror).sendMessage(_payload, _value);
    }
}
