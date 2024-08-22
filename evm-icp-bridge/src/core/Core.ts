import type { Message, Plugin, Router } from "./Types";
import { EnvReader, Keys } from '../utils/envReader';
import { PluginImpl } from "../plugins/common/Plugin";
import * as Evm from '../plugins/evm/index';
import * as Icp from '../plugins/icp/index';
import { ChainId } from "./Types";

export class Core implements Router { 
    plugins: Plugin[] = [];

    async run(): Promise<void> { 
        console.log('Initing Core');
        this.plugins.push(this.setupEvmPlugin());
        this.plugins.push(this.setupIcpPlugin());
     }

    routeMessage(message: Message): void {
       console.log('Message received: ', message.toString());
    }

    setupEvmPlugin(): PluginImpl {
        const rpcProvider = EnvReader.get(Keys.EVM_RPC_PROVIDER);
        const contract = EnvReader.get(Keys.EVM_BRIDGE_CONTRACT_ADDRESS);
        const signerKey = EnvReader.get(Keys.EVM_EXECUTOR_PRIVATE_KEY);

        return new PluginImpl(
            ChainId.Ethereum,
            new Evm.EvmListenerImpl(rpcProvider, contract),
            new Evm.EvmExecutorImpl(rpcProvider, signerKey),
            this
        );
    }

    setupIcpPlugin(): PluginImpl {
        const host = EnvReader.get(Keys.ICP_HOST_URL);
        const canisterId = EnvReader.get(Keys.ICP_CANISTER_ID);
        const secretKey = EnvReader.get(Keys.ICP_EXECUTOR_SECRET_KEY);

        return new PluginImpl(
            ChainId.ICP,
            new Icp.IcpListenerImpl(),
            new Icp.IcpExecutorImpl(host, canisterId, secretKey),
            this
        );
    }
}