# Bitcode Blockchain

Bitcode Blockchain is a custom-built, modular, and educational blockchain written in Rust. It supports:

- Basic proof-of-work mining
- Digital signature validation (Ed25519)
- Transaction broadcast and P2P sync
- JSON-based web dashboard and wallet CLI
- Optional smart contract execution (WASM-ready)

## 🗂 Project Structure

bitcode-blockchain/
├── src/               # Core Rust blockchain logic
├── contracts/         # WASM-based Bitcode-20 smart contracts
├── docs/              # Architecture & protocol documentation
├── .github/           # GitHub Actions, workflows
├── build.rs           # Custom build logic (optional)
├── Cargo.toml         # Crate configuration
├── README.md          # Project overview

# Bitcode Blockchain

[![Rust](https://github.com/bitcode-org/bitcode-blockchain/actions/workflows/rust.yml/badge.svg)](https://github.com/bitcode-org/bitcode-blockchain/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/bitcode-node)](https://crates.io/crates/bitcode-node)
[![Docs.rs](https://docs.rs/bitcode-node/badge.svg)](https://docs.rs/bitcode-node)

Bitcode Blockchain is a minimal, modular, and educational Rust-powered blockchain built from scratch — complete with:
- CLI Wallet
- Web Dashboard
- Peer-to-Peer Sync
- WASM Smart Contract Support
- Faucet Mining, Auto Sync, and Token Standards

  ---
  
## 🧭 Roadmap
- [x] Minimal blockchain core
- [x] Wallet CLI
- [x] WASM smart contract support
- [ ] Dashboard UI (React or Tauri)
- [ ] Bitcode Explorer (Frontend)
- [ ] GitHub Actions CI/CD
- [ ] Token faucet endpoin


## 📘 Documentation
All project guides are available under [`/docs`](./docs):

- [Architecture](docs/architecture.md)
- [Usage Guide](docs/usage.md)
- [Networking Protocols](docs/networking.md)

View index: [`docs/index.md`](docs/index.md)

## 🚀 Quickstart
```bash
cargo build --release
cargo run
```
Then visit `http://localhost:8081` to access the dashboard.

## 🧪 Wallet CLI
```bash
cargo run --example wallet
```

- Generate keypair
- Sign transaction string: `from+to+amount`

## 🧠 Learn More
- [Node Architecture](docs/architecture.md)
- [WebSocket/gRPC Roadmap](docs/networking.md)
- [Bitcode-20 Contracts](contracts/bitcode20)

---

## ❤️ Contributing
All contributions welcome!

### 🛠 Build & Test
```bash
cargo test
cargo clippy
```

### 📦 Publish (crates.io)
- Update `Cargo.toml`
- Login with: `cargo login <TOKEN>`
- Then: `cargo publish`

---

## 📄 License
Licensed under MIT. See [`LICENSE`](LICENSE).

---
Made with ♥ by the Bitcode Core Team.

## Getting Started

To run the blockchain:

```bash
cargo run
