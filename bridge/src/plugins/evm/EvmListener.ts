import { JsonRpcProvider, type AddressLike, type BytesLike } from 'ethers';
import type { Message, Listener, MessageCallback } from '../../core/Types';
import { EvmMetadataReaderImpl } from './utils/EvmMetadataReader';
import { getChainId } from '../../core/Utils';

import { BridgeMediator__factory } from '../../../types/ethers-contracts/factories/BridgeMediator__factory';
import type { BridgeMediator } from '../../../types/ethers-contracts/BridgeMediator';

export class EvmListenerImpl implements Listener {
    private rpcUrl: string;
    private contract: AddressLike;
    private metadataReader: EvmMetadataReaderImpl;

    constructor(rpcUrl: string, contract: AddressLike) {
        this.rpcUrl = rpcUrl;
        this.contract = contract;
        this.metadataReader = new EvmMetadataReaderImpl(rpcUrl);
    }

    async setup(onMessageReceived: MessageCallback): Promise<void> {
        const provider = new JsonRpcProvider(this.rpcUrl, undefined, { staticNetwork: true });
        const bridgeMediator = BridgeMediator__factory.connect(this.contract.toString(), provider);
       
        bridgeMediator.on(bridgeMediator.filters.MessageSend, async (id: BytesLike, messageData: BridgeMediator.MessageStruct) => {      
            console.info('Message received from the EVM', id);
            try {
                const metadata = await this.metadataReader.readMetadata(
                    messageData.contractAddress,
                    Number(messageData.tokenId
                ));
                const { name, symbol } = await this.metadataReader.readCollectionInfo(messageData.contractAddress);
                const msg = this.parseMessage(id, messageData);
                onMessageReceived(msg, name, symbol, metadata);
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