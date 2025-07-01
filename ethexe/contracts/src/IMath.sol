// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.26;

import {IMirrorDecoder} from "./IMirrorDecoder.sol";

interface IMath is IMirrorDecoder {

    event MathMinReply (
        uint256 payload,
        address _destination,
        uint128 _value,
        bytes32 _replyTo,
        bytes4 _replyCode
    );

    event MathSqrtReply (
        uint256 payload,
        address _destination,
        uint128 _value,
        bytes32 _replyTo,
        bytes4 _replyCode
    );

    event OnReplyEvent(bytes payload, address _destination, uint128 _value, bytes32 _replyTo, bytes4 _replyCode);
    event OnMessageEvent(bytes payload, bytes32 _id, address _destination, uint128 _value);
    event ErrorReply(bytes payload, address _destination, uint128 _value, bytes32 _replyTo, bytes4 _replyCode);

    function fnMathMin(uint256 x, uint256 y, uint128 _value) external payable;
    function fnMathSqrt(uint256 y, uint128 _value) external payable;
    function sendMessage(bytes memory _payload, uint128 _value) external payable;
}
