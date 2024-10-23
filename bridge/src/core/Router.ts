import type { Core } from "./Core";
import type { Router, ExtendedMessage } from "./Types";

export class RouterImpl implements Router {
    private core: Core;

    constructor(core: Core) {
        this.core = core;
    }

    // TODO Return the error code and message for the HTTP request in case this fails
    routeMessage(message: ExtendedMessage): void {
        const plugin = this.core.getPlugin(message.destChainId);
        if (!plugin) {
            console.error('No plugin found for the destination chain');
            return;
        }
        plugin.executor.execute(message);
    }
}