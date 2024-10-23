import type { Executor, ExtendedMessage } from '../../core/Types';
import { ethers, JsonRpcProvider, Wallet } from 'ethers';
import { BridgeMediator__factory } from '../../../types/ethers-contracts/factories/BridgeMediator__factory';
import type { BridgeMediator } from '../../../types/ethers-contracts/BridgeMediator';

export class EvmExecutorImpl implements Executor {
    private contract: string;
    private wallet: Wallet;

    constructor(rpcUrl: string, contract: string, signerKey: string) {
        this.contract = contract;
        const provider = new JsonRpcProvider(rpcUrl, undefined, { staticNetwork: true });
        this.wallet = new Wallet(signerKey, provider);
    }

    async execute(message: ExtendedMessage): Promise<void> {
        const bridgeMediator = BridgeMediator__factory.connect(this.contract.toString());
        const packedMessage = this.packMessage(message);
        await bridgeMediator.connect(this.wallet).executeMessage(packedMessage);
    }

    packMessage(message: ExtendedMessage): BridgeMediator.MessageStruct {
        return {
            // Align the IDs between EVM and ICP chains
            id: ethers.encodeBytes32String(message.id),
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