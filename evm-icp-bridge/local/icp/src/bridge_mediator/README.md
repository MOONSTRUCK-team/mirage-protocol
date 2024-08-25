## Setup
1. `dfx start --background --clean`  to start the replica
2. `cargo update` in case new dependencies are added
3. `dfx deploy` to deploy the canister
4. `dfx generate` to generate the TS/JS bindings. Use them when istantiating the Actor.

### Writing the canister interface

`bridge_mediator.did` should be written with a syntax known as Candid. Candid is a language that describes the interface of a canister. It is used to generate bindings for the canister in different languages.