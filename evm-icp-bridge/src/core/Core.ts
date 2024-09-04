import type { Plugin, Router } from "./Types";
import { EnvReader, Keys } from '../utils/envReader';
import { PluginImpl } from "../plugins/common/Plugin";
import { RouterImpl } from "./Router";
import { ChainId } from "./Types";

import * as Evm from '../plugins/evm/index';
import * as Icp from '../plugins/icp/index';

export class Core { 
    private plugins: Map<ChainId, Plugin>;
    private router: Router;

    constructor() {
        this.router = new RouterImpl(this);
        this.plugins = new Map<ChainId, Plugin>();
    }

    async run(): Promise<void> { 
        console.log('Initing Core');

        const evmPlugin = this.setupEvmPlugin();
        this.plugins.set(ChainId.Ethereum, evmPlugin);
        const icpPlugin = this.setupIcpPlugin();
        this.plugins.set(ChainId.ICP, icpPlugin);
     }

    setupEvmPlugin(): PluginImpl {
        const rpcProvider = EnvReader.get(Keys.EVM_RPC_PROVIDER);
        const bridgeMediator = EnvReader.get(Keys.EVM_BRIDGE_CONTRACT_ADDRESS);
        const signerKey = EnvReader.get(Keys.EVM_EXECUTOR_PRIVATE_KEY);

        return new PluginImpl(
            ChainId.Ethereum,
            new Evm.EvmListenerImpl(rpcProvider, bridgeMediator),
            new Evm.EvmExecutorImpl(rpcProvider, bridgeMediator, signerKey),
            this.router
        );
    }

    setupIcpPlugin(): PluginImpl {
        const host = EnvReader.get(Keys.ICP_HOST_URL);
        const bridgeMediatorId = EnvReader.get(Keys.ICP_CANISTER_ID);
        const secretKey = EnvReader.get(Keys.ICP_EXECUTOR_SECRET_KEY);
        const port = Number(EnvReader.get(Keys.ICP_LISTENER_PORT));

        return new PluginImpl(
            ChainId.ICP,
            new Icp.IcpListenerImpl(port),
            new Icp.IcpExecutorImpl(host, bridgeMediatorId, secretKey),
            this.router
        );
    }

    getPlugin(chainId: ChainId): Plugin | undefined {
        return this.plugins.get(chainId);
    }
}