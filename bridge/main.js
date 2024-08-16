import { Actor, HttpAgent } from "@dfinity/agent";
import { identity } from "./identity.js";
import { createRequire } from "module";

const require = createRequire(import.meta.url);

const localCanisterIds = require("../icp/example/.dfx/local/canister_ids.json");
const canisterId = localCanisterIds.example_backend.local;

const main = async () => {
  const idlFactory = ({ IDL }) => {
    return IDL.Service({
      get_name: IDL.Func([], [IDL.Text], ["query"]),
      set_name: IDL.Func([IDL.Text], [], []),
    });
  };

  const agent = await HttpAgent.create({
    identity: identity,
    shouldFetchRootKey: true,
    host: "http://127.0.0.1:4943",
  });

  const actor = Actor.createActor(idlFactory, {
    agent,
    canisterId: canisterId,
  });

  console.log(`Current name: ${await actor.get_name()}`);
  console.log(`Seting new name...`);
  await actor.set_name("Jane Doe");
  console.log(`New name: ${await actor.get_name()}`);
};

main();
