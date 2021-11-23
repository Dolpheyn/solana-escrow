# Solana Escrow dApp

https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction

This is built using pure Solana; not using `borsh` for ser/de, not using
`anchor` for other Solana tings.

### Environment Setup

1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively

```shell
cargo build
cargo test
```

### Build and test the program compiled for BPF

```shell
cargo build-bpf
cargo test-bpf
```
