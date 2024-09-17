import type { Message, Listener, MessageCallback } from '../../core/Types';
import { serve } from 'bun';

export class IcpListenerImpl implements Listener {
    private port: number;
    private tlsCertPath: string;
    private tlsKeyPath: string;
    private onMessageReceived: MessageCallback | undefined;

    constructor(port: number, tlsCertPath: string, tlsKeyPath: string) {
        this.port = port;
        // TODO Extend with the ability to read the content of these from env secrets
        this.tlsCertPath = tlsCertPath;
        this.tlsKeyPath = tlsKeyPath;
    }

    setup(onMessageReceived: MessageCallback): void {
        this.onMessageReceived = onMessageReceived;

        const server = serve({
            port: this.port,
            tls: {
                cert: Bun.file(this.tlsCertPath),
                key: Bun.file(this.tlsKeyPath),
            },
            fetch: async (req: Request): Promise<Response> => {
                console.log('Request received:', req.url, req.method);
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
                opType: BigInt(message.op_type),
                nonce: BigInt(message.nonce),
                srcChainId: message.src_chain_id,
                destChainId: message.dest_chain_id,
                destAddress: message.dest_address,
                contract: message.contract_address,
                tokenId: BigInt(message.token_id),
            };
            const collectionName = "";
            const collectionSymbol = "";
            const metadata = "";

            this.onMessageReceived(parsedMessage, collectionName, collectionSymbol, metadata);
            return new Response('success', { status: 200 });
        } catch (e) {
            console.error(e);
            return new Response('server_side_error', { status: 400 });
        }
    }
}