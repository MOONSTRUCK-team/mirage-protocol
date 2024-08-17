import type { Message, Plugin, Router } from "./Types";
import { EnvReader } from '../utils/envReader';
import * as Evm from '../plugins/evm/index';

export class Core implements Router { 
    plugins: Plugin[] = [];

    constructor() {
        console.log('Initing Core');
        this.plugins.push(this.setupEvmPlugin());
        console.log('Core is ready');

    }
    routeMessage(message: Message): void {
       console.log('Message received: ', message.toString());
    }

    setupEvmPlugin(): Evm.EvmPlugin {
        const rpcProvider = EnvReader.get('EVM_RPC_PROVIDER');
        const contract = EnvReader.get('EVM_BRIDGE_CONTRACT_ADDRESS');
        const signerKey = EnvReader.get('EVM_EXECUTOR_PRIVATE_KEY');

        return new Evm.EvmPlugin(
            new Evm.EvmListenerImpl(rpcProvider, contract),
            new Evm.EvmExecutorImpl(rpcProvider, signerKey),
            this
        );
    }

    setupIcpPlugin(): void {
        // Implement
    }
}