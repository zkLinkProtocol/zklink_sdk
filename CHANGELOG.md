# Changelog
Unreleased

## [3.0.0] - 2023-11-08
### Added
- Defined data types including:
  - basic types like `ChainId`,`Nonce`, `AccountId`, `SubAccountId`, `TokenId` and so on;
  - transaction types:`ChangePubkey`, `Deposit`, `Withdraw`, `ForcedExit`, `FullExit`, `OrderMatching`, `Transfer`;
  - contract transaction types: `AutoDeleveraging`, `ContractMatching`, `Funding`, `Liquidation`;
  - the builder to build the transaction types;
  - json rpc request and response types;
  - other types associated with signing, like private key, public key, address, signature and hash.
- Implement bytes encoding for the all transaction types.
- Implement zklink signing and Ethereum signing.
- Support `wasm`, `Golang` and `Python` bindings for the all above features.
- Implement Ethereum json rpc signer to interact with the wallet like MetaMask for `wasm` binding.
- Implement the provider and rpc client to connect the zklink server and Implement the `wasm` binding.
- Add Rust and Golang unit tests, add wasm tests
- Add example code in folder `examples`
- Add Makefile to build the bindings and lint the code.
- Add `l2_hash` field in `Deposit`. Replace the original `eth_hash` semantics.

### Changed
- Changed the `eth_hash` field to `Option` in `Deposit`.
- Renamed the `eth_hash` field to `l2_hash` in `Fullexit`.
