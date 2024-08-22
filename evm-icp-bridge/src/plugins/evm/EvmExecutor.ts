import type { Executor } from '../../core/Types';
import { ethers } from 'ethers';

export class EvmExecutorImpl implements Executor {
    private rpcUrl: string;
    private wallet: ethers.Wallet;

    constructor(rpcUrl: string, signerKey: string) {
        this.rpcUrl = rpcUrl;
        this.wallet = new ethers.Wallet(signerKey);
    }

    execute(): void {
        throw new Error('Method not implemented.');
    }
}