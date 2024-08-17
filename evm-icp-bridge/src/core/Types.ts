import { type AddressLike } from "ethers"

export interface EvmListener {
    rpcUrl: string;
    contract: AddressLike;
    setup(onMessageReceivedCb: any): void;
}

// Will need to implement a webhook to be triggered by the ICP chain
export interface IcpListener {}

export interface Message {
    data: any;
    sender: string;
    destinationChain: ChainIdentifier;
}

export interface Executor {
    execute(): void;
}

export interface Plugin {
    identifier: ChainIdentifier;
    listener: EvmListener | IcpListener;
    router: Router;
    executor: Executor;
}

export interface Router {
    routeMessage(message: Message): void;
}

export enum ChainIdentifier { EVM, ICP }