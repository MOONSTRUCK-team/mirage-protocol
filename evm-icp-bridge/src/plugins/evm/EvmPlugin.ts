import type { Plugin, EvmListener, Executor, Router, Message } from '../../core/Types';
import { ChainId } from '../../core/Types';

export class EvmPlugin implements Plugin { 
    identifier: ChainId;
    listener: EvmListener;
    executor: Executor;
    router: Router;

    constructor(listener: EvmListener, executor: Executor, router: Router) {
        this.listener = listener;
        this.executor = executor;
        this.router = router;
        this.identifier = ChainId.Mainnet;
        this.listener.setup(this.onMessageReceived);
    }

    onMessageReceived(message: Message): void {
        this.router.routeMessage(message)
    }
}