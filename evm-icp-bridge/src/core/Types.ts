import { type AddressLike } from "ethers"


export interface Plugin {
    identifier: ChainId;
    listener: EvmListener | IcpListener;
    router: Router;
    executor: Executor;
}

export enum ChainId { Mainnet = 1, ICP = "ICP" }

export interface Message {
    id: bigint;
    nonce: bigint;
    srcChainId: ChainId;
    destChainId: ChainId;
    contract: AddressLike;
    tokenId: bigint;
}

export interface Executor {
    execute(): void;
}

export interface Router {
    routeMessage(message: Message): void;
}


export interface EvmListener {
    rpcUrl: string;
    contract: AddressLike;
    setup(onMessageReceivedCb: any): void;
}

// Will need to implement a webhook to be triggered by the ICP chain
export interface IcpListener {}