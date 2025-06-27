
# Bitcode Blockchain Usage Guide

## Prerequisites
- Rust (latest stable)
- Open ports: `8080` for P2P, `8081` for Dashboard

## Run a node
```bash
cargo run
```

## View Web Dashboard
- Open: `http://localhost:8081`
- Submit transaction via form
- View chain or pending TXs as JSON

## CLI Wallet
```bash
cargo run --example wallet
```
- Generate public/private keys
- Sign a TX message string

## Manual TX Submit (example JSON):
```json
{
  "from": "base64_pubkey",
  "to": "recipient_address",
  "amount": 100,
  "fee": 1,
  "signature": "base64_signature"
}
```

Submit via Dashboard `/submit` form or via POST.
