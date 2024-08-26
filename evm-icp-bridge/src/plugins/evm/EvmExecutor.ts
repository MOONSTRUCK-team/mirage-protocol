import type { Executor, Message } from '../../core/Types';
import { JsonRpcProvider, Wallet } from 'ethers';
import { EvmBridgeMediator__factory } from '../../../types/ethers-contracts/factories/EvmBridgeMediator__factory';
import type { BridgeMediator } from '../../../types/ethers-contracts/EvmBridgeMediator';

export class EvmExecutorImpl implements Executor {
    private rpcUrl: string;
    private contract: string;
    private wallet: Wallet;

    constructor(rpcUrl: string, contract: string, signerKey: string) {
        this.rpcUrl = rpcUrl;
        this.contract = contract;
        this.wallet = new Wallet(signerKey);
    }

    async execute(message: Message): Promise<void> {
        const provider = new JsonRpcProvider(this.rpcUrl, undefined, { staticNetwork: true });
        const bridgeMediator = EvmBridgeMediator__factory.connect(this.contract.toString(), provider);

        const packedMessage = this.packMessage(message);
        await bridgeMediator.connect(this.wallet).executeMessage(packedMessage);
        console.log('Message send to the EVM:', packedMessage.id);
    }

    packMessage(message: Message): BridgeMediator.MessageStruct {
        return {
            id: message.id,
            nonce: message.nonce.toString(),
            opType: message.opType.toString(),
            srcChainId: message.srcChainId.toString(),
            destChainId: message.destChainId.toString(),
            destAddress: message.destAddress,
            contractAddress: message.contract.toString(),
            tokenId: message.tokenId.toString(),
        }   
    }
}