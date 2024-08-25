import type { Plugin, Listener, Executor, Router, Message } from '../../core/Types';
import { ChainId } from '../../core/Types';

export class PluginImpl implements Plugin { 
    identifier: ChainId;
    listener: Listener;
    executor: Executor;
    router: Router;

    constructor(chainIdentifier: ChainId, listener: Listener, executor: Executor, router: Router) {
        this.listener = listener;
        this.executor = executor;
        this.router = router;
        this.identifier = chainIdentifier;

        this.onMessageReceived.bind(this);
        this.listener.setup(this.onMessageReceived);
    }

    onMessageReceived = (message: Message): void => {
        this.router.routeMessage(message);
    }
}