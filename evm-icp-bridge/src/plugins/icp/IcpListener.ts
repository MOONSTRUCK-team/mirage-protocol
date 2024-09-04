import type { Message, Listener, MessageCallback } from '../../core/Types';
import { serve } from 'bun';

export class IcpListenerImpl implements Listener {
    private port: number;
    private onMessageReceived: MessageCallback | undefined;

    constructor(port: number) {
        this.port = port;
    }

    setup(onMessageReceived: MessageCallback): void {
        this.onMessageReceived = onMessageReceived;
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
        if (!this.onMessageReceived) {
            console.error('Callback function not set');
            return new Response('server_side_error', { status: 400 });
        }

        if(req.headers.get('content-type') !== 'application/json') {
            return new Response('invalid_content_type', { status: 400 });
        }

        const message =  await req.json();
        try {
            // TODO Make a matching types for EVM and ICP messages (uint256 cannot be represented on ICP)
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

            this.onMessageReceived(parsedMessage);
            return new Response('success', { status: 200 });
        } catch (e) {
            console.error(e);
            return new Response('server_side_error', { status: 400 });
        }
    }
}