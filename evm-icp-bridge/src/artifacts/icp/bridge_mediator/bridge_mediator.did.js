export const idlFactory = ({ IDL }) => {
  const Message = IDL.Record({
    'id' : IDL.Text,
    'dest_chain_id' : IDL.Nat64,
    'src_chain_id' : IDL.Nat64,
    'token_id' : IDL.Nat64,
    'dest_address' : IDL.Text,
    'nonce' : IDL.Nat64,
    'contract_address' : IDL.Text,
    'op_type' : IDL.Nat8,
  });
  return IDL.Service({ 'execute_message' : IDL.Func([Message], [], []) });
};
export const init = ({ IDL }) => { return []; };
