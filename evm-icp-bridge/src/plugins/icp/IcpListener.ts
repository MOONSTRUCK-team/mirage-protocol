import type { Message, Listener } from '../../core/Types';
import { serve } from 'bun';

export class IcpListenerImpl implements Listener {
    private port: number;
    private onMessageReceivedCb: ((message: Message) => void) | undefined;

    constructor(port: number) {
        this.port = port;
    }

    setup(onMessageReceivedCb: (message: Message) => void): void {
        this.onMessageReceivedCb = onMessageReceivedCb;
        const server = serve({
            port: this.port,
            fetch: async (req: Request): Promise<Response> => {
                const { method, url } = req;
                const { pathname } = new URL(url);
                if (method === 'POST' && pathname === '/message') {
                    return await this.handleMessageReceived(req);
                }
                return new Response("not_found", { status: 404 });
            }
         });
        console.log('IcpListener is running on', server.url.toString());
    }

    async handleMessageReceived(req: Request): Promise<Response> {
        if (!this.onMessageReceivedCb) {
            console.error('Callback function not set');
            return new Response('server_side_error', { status: 400 });
        }

        const message =  await req.json();
        try {
            const parsedMessage: Message = {
                id: String(message.id),
                opType: BigInt(message.opType),
                nonce: BigInt(message.nonce),
                srcChainId: message.srcChainId,
                destChainId: message.destChainId,
                destAddress: message.destAddress,
                contract: message.contract,
                tokenId: BigInt(message.tokenId),
            };

            this.onMessageReceivedCb(parsedMessage);
            return new Response('success', { status: 200 });
        } catch (e) {
            console.error(e);
            return new Response('server_side_error', { status: 400 });
        }   
    }
}