
import { JsonRpcProvider, type AddressLike } from 'ethers';
import { ERC721__factory } from '../../../../types/ethers-contracts';

export class EvmMetadataReaderImpl {
    private rpcUrl: string;

    constructor(rpcUrl: string) {
        this.rpcUrl = rpcUrl;
    }

    async readMetadata(contract: AddressLike, tokenId: number): Promise<string> {
        let metadata = '';
        const provider = new JsonRpcProvider(this.rpcUrl, undefined, { staticNetwork: true });
        const nftContract = ERC721__factory.connect(contract.toString(), provider);
        const uri = await nftContract.tokenURI(tokenId);

        
        try {
            // Check if the URI is a valid URL
            const url = new URL(uri);
            const supportedProtocols = ['https:', 'ipfs:'];
            if (supportedProtocols.includes(url.protocol)) {
                const res = await fetch(uri);
                metadata = await res.json();
            }
        } catch {
            // Otherwise assume its raw metadata
            // TODO: Check if any encoding is present
            metadata = uri;
        }
        return metadata;
    }

    async readCollectionInfo(contract: AddressLike): Promise<{ name: string, symbol: string }> {
        const provider = new JsonRpcProvider(this.rpcUrl, undefined, { staticNetwork: true });
        const nftContract = ERC721__factory.connect(contract.toString(), provider);
        const name = await nftContract.name();
        const symbol = await nftContract.symbol();

        return { name, symbol };
    }
}