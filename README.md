# ethers.rs examples

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [solc](https://docs.soliditylang.org/en/latest/installing-solidity.html)
- [anvil](https://book.getfoundry.sh/getting-started/installation)

## 1. Send a transaction

```bash
$ cargo run --bin transact_anvil
```

## 2. Deploy Counter.sol

```bash
$ cargo run --bin deploy_counter
```

> warning: ethers.rs will be [deprecated](https://github.com/gakonst/ethers-rs/issues/2667). Instead, alloy