import type { Message, Listener } from '../../core/Types';

export class IcpListenerImpl implements Listener {
    setup(onMessageReceivedCb: (message: Message) => void): void {
        console.log('ICP Listener not implemented yet');
    }
}