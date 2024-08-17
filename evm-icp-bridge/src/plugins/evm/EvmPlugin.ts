import type { Plugin, EvmListener, Executor, Router, Message } from '../../core/Types';
import { ChainIdentifier } from '../../core/Types';

export class EvmPlugin implements Plugin { 
    identifier: ChainIdentifier;
    listener: EvmListener;
    executor: Executor;
    router: Router;

    constructor(listener: EvmListener, executor: Executor, router: Router) {
        this.listener = listener;
        this.executor = executor;
        this.router = router;
        this.identifier = ChainIdentifier.EVM;
        this.listener.setup(this.onMessageReceived);
    }

    onMessageReceived(message: Message): void {
        this.router.routeMessage(message)
    }
}