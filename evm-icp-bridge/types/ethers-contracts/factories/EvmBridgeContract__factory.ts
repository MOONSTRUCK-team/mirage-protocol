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
        indexed: true,
        name: "from",
        type: "address",
      },
      {
        indexed: true,
        name: "to",
        type: "address",
      },
      {
        indexed: false,
        name: "message",
        type: "string",
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
