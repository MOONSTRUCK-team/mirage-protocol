/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Contract, Interface, type ContractRunner } from "ethers";
import type {
  EvmBridgeContract,
  EvmBridgeContractInterface,
} from "../EvmBridgeContract";

const _abi = [
  {
    anonymous: false,
    inputs: [
      {
        internalType: "uint256",
        name: "id",
        type: "uint256",
        indexed: true,
      },
      {
        components: [
          {
            internalType: "uint256",
            name: "nonce",
            type: "uint256",
          },
          {
            internalType: "uint256",
            name: "srcChainId",
            type: "uint256",
          },
          {
            internalType: "uint256",
            name: "destChainId",
            type: "uint256",
          },
          {
            internalType: "string",
            name: "destAddress",
            type: "string",
          },
          {
            internalType: "uint256",
            name: "tokenId",
            type: "uint256",
          },
          {
            internalType: "address",
            name: "contract",
            type: "address",
          },
        ],
        internalType: "struct Bridge.Message",
        name: "messageData",
        type: "tuple",
      },
    ],
    name: "messageSend",
    type: "event",
  },
] as const;

export class EvmBridgeContract__factory {
  static readonly abi = _abi;
  static createInterface(): EvmBridgeContractInterface {
    return new Interface(_abi) as EvmBridgeContractInterface;
  }
  static connect(
    address: string,
    runner?: ContractRunner | null
  ): EvmBridgeContract {
    return new Contract(address, _abi, runner) as unknown as EvmBridgeContract;
  }
}