import { type AddressLike } from "ethers"

export interface Plugin {
    identifier: ChainId;
    listener: Listener;
    router: Router;
    executor: Executor;
}

// TODO Define the ids for the chains (maybe make them unique, hashes of the names?)
// EVM chains have unique ids, but ICP is not EVM based and does not have a chain id
// The registry of supported chains should be maintained in the core and propably on-chain (but can be skipped, just do not forward the message)
// TODO Remove localhost node support from production code
export enum ChainId { Localhost = 31337, Ethereum = 1, ICP = 2 }

export enum OpType { Mint = 1, Burn = 2 }

export interface Message {
    id: string;
    nonce: bigint;
    opType: bigint;
    srcChainId: ChainId;
    destChainId: ChainId;
    destAddress: string;
    contract: AddressLike;
    tokenId: bigint;
}

export type ExtendedMessage = Message & { collectionName: string, collectionSymbol: string, metadata?: string };

export interface Executor {
    execute(message: ExtendedMessage): void;
}

export interface Router {
    routeMessage(message: ExtendedMessage): void;
}

export type MessageCallback = (message: Message, collectionName: string, collectionSymbol: string, metadata?: string) => void;

export interface Listener {
    setup(onMessageReceived: MessageCallback): void;
}