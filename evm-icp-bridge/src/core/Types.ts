import { type AddressLike } from "ethers"


export interface Plugin {
    identifier: ChainId;
    listener: Listener;
    router: Router;
    executor: Executor;
}

// TODO Define the ids for the chains
// EVM chains have unique ids, but ICP is not EVM based and does not have a chain id
// The registry of supported chains should be maintained in the core and propably on-chain (but can be skipped, just do not forward the message)
export enum ChainId { Ethereum = 1, ICP = 999999 }

export interface Message {
    id: bigint;
    nonce: bigint;
    srcChainId: ChainId;
    destChainId: ChainId;
    destAddress: string;
    contract: AddressLike;
    tokenId: bigint;
}

export interface Executor {
    execute(): void;
}

export interface Router {
    routeMessage(message: Message): void;
}

export interface Listener {
    setup(onMessageReceivedCb: any): void;
}