pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {IRouter} from "../src/IRouter.sol";
import {IWrappedVara} from "../src/IWrappedVara.sol";
import {IMirror} from "../src/IMirror.sol";
import {IMirrorDecoder} from "../src/IMirrorDecoder.sol";

import {Math} from "../src/Math.sol";
import {IMath} from "../src/IMath.sol";

contract DeployMathScript is Script {
    bytes initPayload = hex"0c4e6577";
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address routerAddress = vm.envAddress("ROUTER");
        bytes32 codeId = vm.envBytes32("CODE_ID");
        vm.startBroadcast(privateKey);

        IRouter router = IRouter(routerAddress);
        

        address wrappedVara = router.wrappedVara();  

        bytes32 salt = keccak256(abi.encodePacked(block.timestamp, blockhash(block.number - 1)));
        (address mirror, address decoder) = createContract(router, codeId, salt, wrappedVara);

        vm.stopBroadcast();
    }

    function createContract(IRouter router, bytes32 codeId, bytes32 salt, address wrappedVara)
        private
        returns (address mirrorAddress, address decoderAddress)
    {
        Math math = new Math();

        address mirror = router.createProgramWithDecoder(address(math), codeId, salt);

        console.log("Mirror address: ", mirror);

        IMirror mirrorContract = IMirror(mirror);

        approveWVARA(mirror, wrappedVara);

        mirrorContract.executableBalanceTopUp(100000000000000);

        mirrorContract.sendMessage(initPayload, 0);

        address decoder = mirrorContract.decoder();

        console.log("Decoder: ", decoder);

        require(IMirrorDecoder(decoder).mirror() == mirror, "Script: actual mirror and decoder mirror mismatch");

        return (mirror, decoder);
    }

    function approveWVARA(address account, address wrappedVara) private {
        IWrappedVara(wrappedVara).approve(account, type(uint256).max);
    }
}

