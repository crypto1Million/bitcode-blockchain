# Networking in Bitcode Blockchain

## Message Types:
- `TX:<json>` — Broadcast a new transaction
- `BLOCK:<json>` — Broadcast a newly mined block
- `CHAIN_REQUEST` — Ask peer for full chain
- `CHAIN_RESPONSE:<json>` — Response with full chain
- `PING` — Simple keepalive

## Syncing
- Nodes request and validate longest chain
- Auto-sync every X seconds via `start_auto_sync`

## Peer Discovery
- Load from `peers.txt`
- Plan: DNS discovery, NAT punching

## WebSocket/gRPC (Planned)
- Use `tungstenite` for WebSocket based relays
- Future: Switch to `tonic` or `nats` for scalable mesh

---

To create these files:
```bash
mkdir -p docs
nano docs/architecture.md
nano docs/usage.md
nano docs/networking.md
```
