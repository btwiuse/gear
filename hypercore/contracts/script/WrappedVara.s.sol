// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Script, console} from "forge-std/Script.sol";
import {WrappedVara} from "../src/WrappedVara.sol";

contract WrappedVaraScript is Script {
    function setUp() public {}

    function run() public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        vm.broadcast(privateKey);
        console.log(address(new WrappedVara(vm.addr(privateKey), 6)));
    }
}