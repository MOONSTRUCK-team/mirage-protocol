import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type ExecuteError = { 'MessageNotExecuted' : string };
export type ExecuteResult = { 'Ok' : null } |
  { 'Err' : ExecuteError };
export interface Message {
  'id' : string,
  'dest_chain_id' : bigint,
  'src_chain_id' : bigint,
  'token_id' : bigint,
  'dest_address' : string,
  'nonce' : bigint,
  'contract_address' : string,
  'op_type' : number,
  'token_metadata' : string,
}
export interface _SERVICE {
  'execute_message' : ActorMethod<[Message], undefined>,
  'send_message' : ActorMethod<[], ExecuteResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
