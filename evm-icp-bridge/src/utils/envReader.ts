export enum Keys {
    // EVM related keys
    EVM_RPC_PROVIDER = 'EVM_RPC_PROVIDER',
    EVM_BRIDGE_CONTRACT_ADDRESS = 'EVM_BRIDGE_CONTRACT_ADDRESS',
    EVM_EXECUTOR_PRIVATE_KEY = 'EVM_EXECUTOR_PRIVATE_KEY',
    // ICP related keys
    ICP_HOST_URL = 'ICP_HOST_URL',
    ICP_CANISTER_ID = 'ICP_CANISTER_ID',
    ICP_EXECUTOR_SECRET_KEY = 'ICP_EXECUTOR_SECRET_KEY',
    ICP_LISTENER_PORT = 'ICP_LISTENER_PORT',
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

