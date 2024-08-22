import { ethers, type AddressLike } from 'ethers';
import { type Message, type EvmListener , ChainId} from '../../core/Types';
import { EvmBridgeContract__factory } from '../../../types/ethers-contracts/factories/EvmBridgeContract__factory';
import type { Bridge } from '../../../types/ethers-contracts/EvmBridgeContract';
import type { BigNumberish } from 'ethers';

export class EvmListenerImpl implements EvmListener {
    rpcUrl: string;
    contract: AddressLike;

    constructor(rpcUrl: string, contract: AddressLike) {
        this.rpcUrl = rpcUrl;
        this.contract = contract;
    }

    setup(onMessageReceivedCb: (message: Message) => void): void {
        const provider = new ethers.JsonRpcProvider(this.rpcUrl);
        const contractInstance = EvmBridgeContract__factory.connect(this.contract.toString(), provider);

        contractInstance.on(contractInstance.filters.messageSend, (id: BigNumberish, messageData: Bridge.MessageStruct) => {
            const msg = this.parseMessage(id, messageData);
            onMessageReceivedCb(msg);
        });

        console.log('EVM Listener is set up');
    }

    parseMessage(messageId: BigNumberish, message: Bridge.MessageStruct): Message {
        return {
            // TODO Check if this will properly parse the BigInt values
            id: BigInt(messageId),
            nonce: BigInt(message.nonce),
            srcChainId: this.getChainId(BigInt(message.srcChainId)),
            destChainId: ChainId.ICP,
            contract: message.contract,
            tokenId: BigInt(message.tokenId)
        };
    }

    getChainId(chainId: bigint): ChainId {
        if (chainId === BigInt(1)) {
            return ChainId.Mainnet;
        } else {
            // TODO Define the magic ICP chain ID
            return ChainId.ICP;
        }
    }
}