import type { Executor } from '../../core/Types';
import { ethers } from 'ethers';

export class EvmExecutorImpl implements Executor {
    rpcUrl: string;
    wallet: ethers.Wallet;

    constructor(rpcUrl: string, signerKey: string) {
        this.rpcUrl = rpcUrl;
        this.wallet = new ethers.Wallet(signerKey)
    }

    execute(): void {
        const provider = new ethers.JsonRpcProvider(this.rpcUrl);
        this.wallet.connect(provider);
        // execute the message

    }
}