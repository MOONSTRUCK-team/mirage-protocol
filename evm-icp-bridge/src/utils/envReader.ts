export enum Keys {
    EVM_RPC_PROVIDER = 'EVM_RPC_PROVIDER',
    EVM_BRIDGE_CONTRACT_ADDRESS = 'EVM_BRIDGE_CONTRACT_ADDRESS',
    EVM_EXECUTOR_PRIVATE_KEY = 'EVM_EXECUTOR_PRIVATE_KEY'
}

export class EnvReader {
    static get(key: Keys): string {
        const value = process.env[key];
        
        if (!value) {
            throw new Error(`Missing required environment variable ${key}`);
        }

        return value;
    }
}

