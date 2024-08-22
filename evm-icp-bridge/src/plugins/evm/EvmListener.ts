import { JsonRpcProvider, type AddressLike, type BigNumberish } from 'ethers';
import { type Message, type Listener, ChainId} from '../../core/Types';
import { EvmBridgeContract__factory } from '../../../types/ethers-contracts/factories/EvmBridgeContract__factory';
import type { Bridge } from '../../../types/ethers-contracts/EvmBridgeContract';
import { getChainId } from '../../core/Utils';

export class EvmListenerImpl implements Listener {
    private rpcUrl: string;
    private contract: AddressLike;

    constructor(rpcUrl: string, contract: AddressLike) {
        this.rpcUrl = rpcUrl;
        this.contract = contract;
    }

    setup(onMessageReceivedCb: (message: Message) => void): void {
        const provider = new JsonRpcProvider(this.rpcUrl);
        const contractInstance = EvmBridgeContract__factory.connect(this.contract.toString(), provider);

        contractInstance.on(contractInstance.filters.messageSend, (id: BigNumberish, messageData: Bridge.MessageStruct) => {
            try {
                const msg = this.parseMessage(id, messageData);
                onMessageReceivedCb(msg);
            }
            catch (e) {
                console.error(e);
            }
        });

        console.log('EVM Listener is set up');
    }

    parseMessage(messageId: BigNumberish, message: Bridge.MessageStruct): Message {
        return {
            id: BigInt(messageId),
            nonce: BigInt(message.nonce),
            srcChainId: getChainId(Number(message.srcChainId)),
            destChainId: getChainId(Number(message.destChainId)),
            destAddress: message.destAddress,
            contract: message.contract,
            tokenId: BigInt(message.tokenId)
        };
    }
}