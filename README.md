## Getting Started

Build and deploy

```bash
cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=dist

solana program deploy dist/race.so
```
