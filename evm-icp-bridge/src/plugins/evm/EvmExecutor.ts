import type { Executor, ExtendedMessage } from '../../core/Types';
import { JsonRpcProvider, Wallet } from 'ethers';
import { BridgeMediator__factory } from '../../../types/ethers-contracts/factories/BridgeMediator__factory';
import type { BridgeMediator } from '../../../types/ethers-contracts/BridgeMediator';

export class EvmExecutorImpl implements Executor {
    private rpcUrl: string;
    private contract: string;
    private wallet: Wallet;

    constructor(rpcUrl: string, contract: string, signerKey: string) {
        this.rpcUrl = rpcUrl;
        this.contract = contract;
        this.wallet = new Wallet(signerKey);
    }

    async execute(message: ExtendedMessage): Promise<void> {
        const provider = new JsonRpcProvider(this.rpcUrl, undefined, { staticNetwork: true });
        const bridgeMediator = BridgeMediator__factory.connect(this.contract.toString(), provider);

        const packedMessage = this.packMessage(message);
        await bridgeMediator.connect(this.wallet).executeMessage(packedMessage);
        console.log('Message send to the EVM:', packedMessage.id);
    }

    packMessage(message: ExtendedMessage): BridgeMediator.MessageStruct {
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