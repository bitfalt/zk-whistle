# zk-whistle

A civic platform for Costa Rica with two modules:

- **Fact check:** citizens check and rate public statements by the government.
- **Whistleblow:** anyone can send a report without fear of being identified.

One verified person gets one voice in each module. Nothing links that person across the two modules.

> Status: early skeleton. The zk verifier is a stub. Do not use it with real reports yet.

## Layout

```
apps/
  factcheck-web/    Next.js frontend for fact checking (port 3001)
  leaks-web/        Next.js frontend for whistleblowing (port 3002)
crates/
  zk/               Proof verification and scoped nullifiers
  api/              Shared axum router (/health, /auth/verify)
services/
  factcheck-api/    Fact check API (port 8081)
  leaks-api/        Whistleblow API (port 8082)
docker-compose.yml  Local Postgres per module + MinIO
```

## Run locally

Requires Rust (stable), Node 24 and pnpm.

```sh
cargo test --workspace
cargo run -p factcheck-api   # :8081
cargo run -p leaks-api       # :8082

pnpm install
pnpm dev                     # :3001 and :3002

docker compose up            # optional: databases and blob storage
```

Try the verify endpoint:

```sh
curl -s localhost:8081/auth/verify \
  -H 'content-type: application/json' \
  -d '{"identity_commitment":"demo","nationality":"CRI","age_over_18":true}'
```

## Docs

- [ARCHITECTURE.md](ARCHITECTURE.md): design, privacy rules and roadmap.

## License

MIT
