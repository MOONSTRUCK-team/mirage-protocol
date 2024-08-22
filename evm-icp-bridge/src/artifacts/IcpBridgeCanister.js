export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    get_name: IDL.Func([], [IDL.Text], ["query"]),
    set_name: IDL.Func([IDL.Text], [], []),
  });
};
export const init = ({ IDL }) => {
  return [];
};
