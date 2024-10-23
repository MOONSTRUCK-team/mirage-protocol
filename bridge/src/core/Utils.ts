import { ChainId } from "./Types";

export function getChainId(chainId: number): ChainId {
    const str = ChainId[chainId];
    return ChainId[str as keyof typeof ChainId];
}