import type { Message, Plugin, Router } from "./Types";
import { EnvReader, Keys } from '../utils/envReader';
import { PluginImpl } from "../plugins/common/Plugin";
import { RouterImpl } from "./Router";
import * as Evm from '../plugins/evm/index';
import * as Icp from '../plugins/icp/index';
import { ChainId } from "./Types";

export class Core { 
    private plugins = new Map<number, Plugin>();
    private router = new RouterImpl(this);

    async run(): Promise<void> { 
        console.log('Initing Core');
        this.plugins.set(ChainId.Ethereum, this.setupEvmPlugin());
        this.plugins.set(ChainId.ICP, this.setupIcpPlugin());
     }

    setupEvmPlugin(): PluginImpl {
        const rpcProvider = EnvReader.get(Keys.EVM_RPC_PROVIDER);
        const contract = EnvReader.get(Keys.EVM_BRIDGE_CONTRACT_ADDRESS);
        const signerKey = EnvReader.get(Keys.EVM_EXECUTOR_PRIVATE_KEY);

        return new PluginImpl(
            ChainId.Ethereum,
            new Evm.EvmListenerImpl(rpcProvider, contract),
            new Evm.EvmExecutorImpl(rpcProvider, signerKey),
            this.router
        );
    }

    setupIcpPlugin(): PluginImpl {
        const host = EnvReader.get(Keys.ICP_HOST_URL);
        const canisterId = EnvReader.get(Keys.ICP_CANISTER_ID);
        const secretKey = EnvReader.get(Keys.ICP_EXECUTOR_SECRET_KEY);
        const port = Number(EnvReader.get(Keys.ICP_LISTENER_PORT));

        return new PluginImpl(
            ChainId.ICP,
            new Icp.IcpListenerImpl(port),
            new Icp.IcpExecutorImpl(host, canisterId, secretKey),
            this.router
        );
    }

    getPlugin(chainId: ChainId): Plugin | undefined {
        return this.plugins.get(chainId);
    }
}