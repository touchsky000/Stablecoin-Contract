# StableCoin Smart Contract

This project is Staking management system on Solana.
Token is SPL-TOKEN standard.

Try running some of the following tasks:

### Environment

```shell
Node Version: 22
Yarn Version: 1.22.22
Anchor Version: 0.29.0
Rust Version: 1.84.0
Cargo Version: 1.84.0
```

### Module Install

```shell
npm i
```

### Compile

```shell
anchor keys list
anchor keys sync
anchor build
```
The result will be
```shell
   Compiling serde_derive v1.0.217
   Compiling thiserror-impl v1.0.69
   Compiling borsh-derive v1.5.5
   Compiling solana-frozen-abi-macro v1.18.26
   Compiling bytemuck_derive v1.8.1
   Compiling solana-sdk-macro v1.18.26      
   Compiling num-derive v0.4.2
   Compiling spl-program-error-derive v0.3.2
   Compiling num_enum_derive v0.7.3
   Compiling num-derive v0.3.3
   Compiling anchor-derive-space v0.29.0
   Compiling num_enum v0.7.3
   Compiling thiserror v1.0.69
   Compiling spl-discriminator-syn v0.1.2
   Compiling spl-discriminator-derive v0.1.2
   Compiling bytemuck v1.21.0
   Compiling borsh v1.5.5
   Compiling serde v1.0.217
   Compiling bv v0.11.1
   Compiling serde_bytes v0.11.15
   Compiling serde_json v1.0.137
   Compiling bincode v1.3.3
   Compiling toml v0.5.11
   Compiling solana-frozen-abi v1.18.26
   Compiling proc-macro-crate v0.1.5
   Compiling anchor-syn v0.29.0
   Compiling borsh-derive v0.9.3
   Compiling borsh-derive v0.10.4
   Compiling borsh v0.9.3
   Compiling borsh v0.10.4
   Compiling solana-program v1.18.26
   Compiling anchor-attribute-error v0.29.0
   Compiling anchor-attribute-event v0.29.0
   Compiling anchor-attribute-access-control v0.29.0
   Compiling anchor-attribute-program v0.29.0
   Compiling anchor-attribute-account v0.29.0
   Compiling anchor-derive-accounts v0.29.0
   Compiling anchor-attribute-constant v0.29.0
   Compiling anchor-derive-serde v0.29.0
   Compiling spl-program-error v0.3.0
   Compiling solana-zk-token-sdk v1.18.26
   Compiling spl-discriminator v0.1.0
   Compiling spl-token v4.0.3
   Compiling spl-memo v4.0.4
   Compiling mpl-token-metadata v3.2.3
   Compiling spl-pod v0.1.0
   Compiling spl-type-length-value v0.3.0
   Compiling spl-token-group-interface v0.1.0
   Compiling spl-tlv-account-resolution v0.5.1
   Compiling spl-token-metadata-interface v0.2.0
   Compiling spl-tlv-account-resolution v0.4.0
   Compiling anchor-lang v0.29.0
   Compiling spl-transfer-hook-interface v0.3.0
   Compiling spl-transfer-hook-interface v0.4.1
   Compiling spl-token-2022 v1.0.0
   Compiling spl-token-2022 v0.9.0
   Compiling spl-associated-token-account v2.3.0
   Compiling anchor-spl v0.29.0
   Compiling staking-contract v0.1.0 (/mnt/e/development/git/Staking-Contract/programs/staking-contract)
    Finished release [optimized] target(s) in 1m 07s
```
### Test Code Execute

```shell
anchor test
```

The result will be 

```shell

Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "/home/whitehorse/Documents/project/Stablecoin-Contract/Anchor.toml"

yarn run v1.22.22
warning package.json: No license field
$ /home/whitehorse/Documents/project/Stablecoin-Contract/node_modules/.bin/ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'
(node:70267) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)


  stablecoin-contract
Your transaction signature 5bFUDgYkgxGZViTQcG3JenTsaDqGgrxYEdyqtbKu3JkSYcmysDPViv5iGUMmYXpLuAPWc1AfqesMn36ukRinxcxR
    ✔ Is initialized! (180ms)
    ✔ Initialize Global State (1248ms)
    ✔ collate token create (842ms)
    ✔ update mint cap (413ms)
    ✔ update_reserve_ratio (416ms)
    ✔ emergency_pause (456ms)
    ✔ emergency_resume (416ms)
    ✔ deposite_collateral (1249ms)
    ✔ stable coin create (831ms)
    ✔ admin mint stablecoin (830ms)
>> Stable balance >> 100
    ✔ StableCoin balance
Tx =>  5KAUqtvYXFFrCvx2x38GQ6iAFbs3DqDp4GNekFAg19x7HV1WH6Eh73LojWDqgopRM5r6sBmgXcscDH4Zjck1CUke
    ✔ admin redeem (414ms)
>> Stable balance >> 999916
    ✔ token balance
Mint Cap:  200000
Total Minted:  60
Total totalCollateral:  84
    ✔ get Global


  14 passing (7s)

Done in 8.65s.
```