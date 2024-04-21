# A CosmWasm smart contract for dealing with donations

## Problem statement

> You are a smart contract developer working on a donation project. Your platform lets people make donations to any Project, a Project is just about anything under which CW20 Tokens are being collected on the contract.
>
> A Project can have a name and creator. Then any user can come to the contract and donate Tokens under a project.
>
> The contract always Counts the donations received under the Project Via a given User and saves the amount of donation and issues.
>
> This information can be queried with their wallet addresses.
>
> The donations are always sent to the Project creators wallet address.
>
> Contract needs to deduct 10% if the Donation received is less than 10,000 CW 20 TOKENS. 5% if More than that.
>
> Fees collected are sent to the fee collector wallet. Contract MUST have Rust Local or JS tests to showcase functionality.

## Contract

For tests, see the `tests` module in the [`src/contract.rs`](./src/contract.rs) file.

## Running tests

```console
cargo test
```

## Generating the schema

```console
cargo schema
```

## Building the contract

```console
cargo wasm
```

For better results, see <https://docs.osmosis.zone/cosmwasm/testnet/cosmwasm-deployment/#optimized-compilation>.

## Also see

* ["Deploying Cosmwasm Smart Contract on Local testnet" article on Medium](https://medium.com/@vishalpotpelliwar123/deploying-cosmwasm-smart-contract-on-local-testnet-c6a3d973865c).
