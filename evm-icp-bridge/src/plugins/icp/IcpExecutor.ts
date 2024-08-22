import type { Executor } from '../../core/Types';
import { Actor, HttpAgent } from "@dfinity/agent";
import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1";
import { idlFactory } from '../../artifacts/IcpBridgeCanister'
import { fromHexString as hexStingToArrayBuffer } from '@dfinity/candid';

// TODO Write a proper bridge interface and generate the proper output for the factory
export class IcpExecutorImpl implements Executor {
    agent: HttpAgent | undefined;
    actor: Actor | undefined;
    inited: boolean = false;

    constructor(host: string, canisterId: string, secretKey: string) {
        this.setup(host, canisterId, secretKey);
    }

    async setup(host: string, canisterId: string, secretKey: string): Promise<void> {
        const identity = Secp256k1KeyIdentity.fromSecretKey(hexStingToArrayBuffer(secretKey));
        
        this.agent = await HttpAgent.create({
            identity: identity,
            host: host,
            shouldFetchRootKey: true,
        })
        this.actor = Actor.createActor(idlFactory, {
            agent: this.agent,
            canisterId: canisterId,
        });

        this.inited = true;

        console.log('ICP Executor ready');
    }

    execute(): void {
        if (!this.inited) {
            throw new Error('ICP Executor is not fully initialized');
        }
        // Send the message to the other side
    }
}