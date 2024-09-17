export const idlFactory = ({ IDL }) => {
  const Message = IDL.Record({
    'id' : IDL.Text,
    'dest_chain_id' : IDL.Nat64,
    'src_chain_id' : IDL.Nat64,
    'token_id' : IDL.Nat64,
    'dest_address' : IDL.Text,
    'collection_name' : IDL.Text,
    'collection_symbol' : IDL.Text,
    'nonce' : IDL.Nat64,
    'contract_address' : IDL.Text,
    'op_type' : IDL.Nat8,
    'token_metadata' : IDL.Text,
  });
  const ExecuteError = IDL.Variant({ 'MessageNotExecuted' : IDL.Text });
  const ExecuteResult = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ExecuteError });
  return IDL.Service({
    'execute_message' : IDL.Func([Message], [], []),
    'send_message' : IDL.Func([], [ExecuteResult], []),
  });
};
export const init = ({ IDL }) => { return []; };
