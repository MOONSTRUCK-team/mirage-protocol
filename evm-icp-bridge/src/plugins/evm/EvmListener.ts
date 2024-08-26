import { JsonRpcProvider, type AddressLike, type BytesLike } from 'ethers';
import { type Message, type Listener} from '../../core/Types';
import { getChainId } from '../../core/Utils';

import { EvmBridgeMediator__factory } from '../../../types/ethers-contracts/factories/EvmBridgeMediator__factory';
import type { BridgeMediator } from '../../../types/ethers-contracts/EvmBridgeMediator';

export class EvmListenerImpl implements Listener {
    private rpcUrl: string;
    private contract: AddressLike;

    constructor(rpcUrl: string, contract: AddressLike) {
        this.rpcUrl = rpcUrl;
        this.contract = contract;
    }

    setup(onMessageReceivedCb: (message: Message) => void): void {
        const provider = new JsonRpcProvider(this.rpcUrl, undefined, { staticNetwork: true });
        const contractInstance = EvmBridgeMediator__factory.connect(this.contract.toString(), provider);
       
        contractInstance.on(contractInstance.filters.MessageSend, (id: BytesLike, messageData: BridgeMediator.MessageStruct) => {      
            console.info('Message received from the EVM', id);
            try {
                const msg = this.parseMessage(id, messageData);
                onMessageReceivedCb(msg);
            }
            catch (e) {
                console.error(e);
            }
        });

        console.log('EvmListner is listening on', this.rpcUrl);
    }

    parseMessage(messageId: BytesLike, message: BridgeMediator.MessageStruct): Message {
        return {
            id: String(messageId),
            opType: BigInt(message.opType),
            nonce: BigInt(message.nonce),
            srcChainId: getChainId(Number(message.srcChainId)),
            destChainId: getChainId(Number(message.destChainId)),
            destAddress: message.destAddress,
            contract: message.contractAddress,
            tokenId: BigInt(message.tokenId)
        };
    }
}