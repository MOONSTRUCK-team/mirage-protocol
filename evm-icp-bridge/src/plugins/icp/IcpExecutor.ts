import type { Executor, ExtendedMessage } from '../../core/Types';
import { type ActorSubclass, HttpAgent } from "@dfinity/agent";
import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1";
import { createActor } from '../../artifacts/icp';
import { fromHexString as hexStingToArrayBuffer } from '@dfinity/candid';
import type { _SERVICE as BridgeMediator_Service, Message as BridgeMediator_Message } from '../../artifacts/icp/bridge_mediator.did';

export class IcpExecutorImpl implements Executor {
    private agent: HttpAgent | undefined;
    private actor: ActorSubclass<BridgeMediator_Service> | undefined;

    constructor(host: string, canisterId: string, secretKey: string) {
        this.setup(host, canisterId, secretKey);
    }

    async setup(host: string, canisterId: string, secretKey: string): Promise<void> {
        const identity = Secp256k1KeyIdentity.fromSecretKey(hexStingToArrayBuffer(secretKey));
        this.agent = await HttpAgent.create({
            identity: identity,
            host: host,
            shouldFetchRootKey: true,
        });

        this.actor = createActor(canisterId, {
            agent: this.agent,
        });
        console.log('ICP Executor ready');
    }

    async execute(message: ExtendedMessage): Promise<void> {
        if (!this.isInitialized()) {
            throw new Error('ICP Executor is not fully initialized');
        }

        const packedMessage: BridgeMediator_Message = {
            id: message.id,
            nonce: BigInt(message.nonce),
            op_type: Number(message.opType),
            src_chain_id: BigInt(message.srcChainId),
            dest_chain_id: BigInt(message.destChainId),
            dest_address: message.destAddress,
            contract_address: message.contract.toString(),
            collection_name: message.collectionName,
            collection_symbol: message.collectionSymbol,
            token_id: BigInt(message.tokenId),
            token_metadata: message.metadata ?? '',
        }
        try {
            await this.actor?.execute_message(packedMessage);
            console.log('Message executed on the ICP:', message.id);
        } catch (e) {
            console.error(e);
        }
    }

    isInitialized(): boolean {
        return this.actor !== undefined && this.agent !== undefined;
    }
}