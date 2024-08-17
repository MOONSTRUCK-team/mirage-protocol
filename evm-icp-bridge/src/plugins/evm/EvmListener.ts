import { ethers, type AddressLike } from 'ethers';
import { type Message, type EvmListener , ChainIdentifier} from '../../core/Types';
import { EvmBridgeContract__factory } from '../../../types/ethers-contracts';

export class EvmListenerImpl implements EvmListener {
    rpcUrl: string;
    contract: AddressLike;

    constructor(rpcUrl: string, contract: AddressLike) {
        this.rpcUrl = rpcUrl;
        this.contract = contract;
    }

    setup(onMessageReceivedCb: (message: Message) => void): void {
        // setup listener
        const provider = new ethers.JsonRpcProvider(this.rpcUrl);
        const contractInstance = EvmBridgeContract__factory.connect(this.contract.toString(), provider);

        // Set up event listeners or other initialization logic
        contractInstance.on(contractInstance.filters.messageSend, (from, to, message) => {
            const msg = this.parseMessage(message);
            // 1. Create an message object. Can eventData be type checked?
            // 2. Decode the data in order to read the destination chain
            // 3. Define the data format (interface) with message sender
            onMessageReceivedCb(msg);
        });

        console.log('EVM Listener is set up');
    }

    parseMessage(message: string): Message {
        // Decode the message
        // Return a Message object
        return {
            data: message,
            sender: '0x123',
            destinationChain: ChainIdentifier.ICP
        };
    }
}