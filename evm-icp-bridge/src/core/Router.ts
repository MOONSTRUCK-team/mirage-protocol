import type { Core } from "./Core";
import type { Router, ExtendedMessage } from "./Types";

export class RouterImpl implements Router {
    private core: Core;

    constructor(core: Core) {
        this.core = core;
    }

    routeMessage(message: ExtendedMessage): void {
        const plugin = this.core.getPlugin(message.destChainId);
        if (!plugin) {
            console.error('No plugin found for the destination chain');
            return;
        }
        plugin.executor.execute(message);
    }
}