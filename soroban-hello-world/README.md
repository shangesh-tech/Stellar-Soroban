```cli
stellar contract init soroban-hello-world
```
```cli

shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ stellar keys generate alice --network testnet --fund
✅ Key saved with alias alice in "/home/shangesh/.config/stellar/identity/alice.toml"
✅ Account alice funded on "Test SDF Network ; September 2015"
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ stellar keys address alic
❌ error: Failed to find config identity for alic
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ stellar keys address alice
GDCQPBGN4R3R6UR6EM3ZEYLYUU5O6E2A7GKZRTWQPHENVWJOHR66C46Y
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ stellar keys ls -l
/home/shangesh/.config/stellar/identity/alice.toml
Name: alice

shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ 
```
```cli
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ ls
Cargo.lock  contracts  target
Cargo.toml  README.md
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ stellar contract build
ℹ️  CARGO_BUILD_RUSTFLAGS=--remap-path-prefix=/home/shangesh/.cargo/registry/src= cargo rustc --manifest-path=contracts/hello-world/Cargo.toml --crate-type=cdylib --target=wasm32v1-none --release
  Downloaded soroban-env-guest v23.0.1
  Downloaded 1 crate (7.3KiB) in 0.11s
   Compiling syn v2.0.114
   Compiling num-traits v0.2.19
   Compiling escape-bytes v0.1.1
   Compiling static_assertions v1.1.0
   Compiling ethnum v1.5.2
   Compiling darling_core v0.21.3
   Compiling darling_core v0.20.11
   Compiling prettyplease v0.2.37
   Compiling macro-string v0.1.4
   Compiling serde_derive v1.0.228
   Compiling cfg_eval v0.1.2
   Compiling num-derive v0.4.2
   Compiling thiserror-impl v1.0.69
   Compiling visibility v0.1.1
   Compiling bytes-lit v0.0.5
   Compiling thiserror v1.0.69
   Compiling darling_macro v0.20.11
   Compiling darling_macro v0.21.3
   Compiling darling v0.20.11
   Compiling darling v0.21.3
   Compiling serde_with_macros v3.16.1
   Compiling serde v1.0.228
   Compiling crate-git-revision v0.0.6
   Compiling schemars v0.8.22
   Compiling hex v0.4.3
   Compiling stellar-strkey v0.0.13
   Compiling stellar-xdr v23.0.0
   Compiling soroban-env-common v23.0.1
   Compiling soroban-sdk v23.4.1
   Compiling serde_with v3.16.1
   Compiling soroban-spec v23.4.1
   Compiling soroban-spec-rust v23.4.1
   Compiling soroban-env-macros v23.0.1
   Compiling soroban-env-guest v23.0.1
   Compiling soroban-sdk-macros v23.4.1
   Compiling hello-world v0.0.0 (/home/shangesh/Stellar Soroban/soroban-hello-world/contracts/hello-world)
    Finished `release` profile [optimized] target(s) in 21.87s
ℹ️  Build Summary:
    Wasm File: target/wasm32v1-none/release/hello_world.wasm (788 bytes)
    Wasm Hash: 4584ea75e4120d96c248c33a8975657adbab63879fda58a2fc8577552a00a3db
    Wasm Size: 788 bytes
    Exported Functions: 2 found
      • greet
      • hello
✅ Build Complete

shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello_world.wasm \
  --source-account alice \
  --network testnet \
  --alias hello_world
ℹ️  Simulating install transaction…
ℹ️  Signing transaction: e19ddd52815fe1111ddab216a1ab78016495544e16884c89a8df1491d29269b8
🌎 Submitting install transaction…
ℹ️  Using wasm hash 4584ea75e4120d96c248c33a8975657adbab63879fda58a2fc8577552a00a3db
ℹ️  Simulating deploy transaction…
ℹ️  Transaction hash is 63c88b9b823995394bab6a8a66accbefb9380f2a32c13cf4609690bfd1d57e41
🔗 https://stellar.expert/explorer/testnet/tx/63c88b9b823995394bab6a8a66accbefb9380f2a32c13cf4609690bfd1d57e41
ℹ️  Signing transaction: 63c88b9b823995394bab6a8a66accbefb9380f2a32c13cf4609690bfd1d57e41
🌎 Submitting deploy transaction…
🔗 https://lab.stellar.org/r/testnet/contract/CC355T6CFK53EBFCCECZNL2JUFSVNNDNHKOZ74KTV7FOSSQVYDKOT4KJ
✅ Deployed!
CC355T6CFK53EBFCCECZNL2JUFSVNNDNHKOZ74KTV7FOSSQVYDKOT4KJ
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ 
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ stellar contract invoke \
  --id CC355T6CFK53EBFCCECZNL2JUFSVNNDNHKOZ74KTV7FOSSQVYDKOT4KJ \
  --source-account alice \
  --network testnet \
  -- \
  hello \
  --to RPC
ℹ️  Simulation identified as read-only. Send by rerunning with `--send=yes`.
["Hello","RPC"]
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ stellar contract invoke   --id CC355T6CFK53EBFCCECZNL2JUFSVNNDNHKOZ74KTV7FOSSQVYDKOT4KJ   --source-account alice   --network testnet   --   greet   --to RPC
error: unexpected argument '--to' found

Usage: greet [OPTIONS]

For more information, try '--help'.
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ stellar contract invoke   --id CC355T6CFK53EBFCCECZNL2JUFSVNNDNHKOZ74KTV7FOSSQVYDKOT4KJ   --source-account alice   --network testnet   --   greet   --name RPC
ℹ️  Simulation identified as read-only. Send by rerunning with `--send=yes`.
"Hello, Shangesh!"
shangesh@fedora:~/Stellar Soroban/soroban-hello-world$ 
```
# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
