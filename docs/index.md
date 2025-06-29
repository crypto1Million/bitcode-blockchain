# Bitcode Blockchain Documentation Index

Welcome to the Bitcode Blockchain developer and node operator guide.

## 📂 Contents
- [Architecture](architecture.md)
- [Usage Guide](usage.md)
- [Networking Protocols](networking.md)
- [Smart Contracts](../contracts/bitcode20/token.wasm)
  
## 🛠 For Contributors
To build locally:
```bash
cargo build
```
To run tests:
```bash
cargo test
```
To build contracts:
```bash
cd contracts/bitcode20 && cargo build --target wasm32-unknown-unknown --release
```

---
Generated with by the Bitcode Core Team.
