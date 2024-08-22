import type { Executor } from '../../core/Types';
import { Actor, HttpAgent } from "@dfinity/agent";
import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1";
import { idlFactory} from '../../artifacts/IcpBridgeCanister'


// TODO Write a proper bridge interface and generate the proper output for the factory
export class IcpExecutorImpl implements Executor {
    agent: HttpAgent | undefined;
    actor: Actor | undefined;
    inited: boolean = false;

    constructor(host: string, canisterId: string, identityKey: string) {
        this.setup(host, canisterId, identityKey);
    }

    async setup(host: string, canisterId: string, identityKey: string): Promise<void> {
        // TODO - Use the identityKey to generate the identity
        const identity = Secp256k1KeyIdentity.generate();
        
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
    }

    execute(): void {
        if (!this.inited) {
            throw new Error('IcpExecutorImpl is not fully initialized');
        }
        // Send the message to the other side
    }
}