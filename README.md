# proxima

A high-performance HTTP reverse proxy and load balancer built in Rust.

Built as a learning and portfolio project — each milestone ships something runnable, not a half-finished rewrite.

## Features (by milestone)

| # | Milestone | Status |
|---|---|---|
| 1 | TCP passthrough proxy — async I/O, one task per connection | ✅ done |
| 2 | HTTP/1.1 reverse proxy — hyper, request/response forwarding | ✅ done |
| 3 | Multi-upstream routing + load balancing — round-robin, least-conn, health checks | 🔜 |
| 4 | Config hot-reload — SIGHUP, `Arc<RwLock<Config>>` | |
| 5 | TLS termination — rustls inbound HTTPS, optional upstream TLS | |
| 6 | Circuit breaking — per-upstream failure tracking, half-open/open/closed FSM | |
| 7 | Observability — Prometheus `/metrics`, p50/p95/p99 latency, structured access logs | |
| 8 | HTTP/2 — h2 on inbound and upstream via hyper | |

## Stack

- **Rust** (async via tokio)
- **hyper** — HTTP/1.1 and HTTP/2
- **rustls** — TLS termination
- **Prometheus** metrics endpoint
- **TOML** config file

## Running

```bash
cargo build --release
./target/release/proxima
```

Listens on `0.0.0.0:8080` and proxies to the configured upstream.

## Architecture

- One tokio task per connection — connection-oriented concurrency, not thread-per-request
- Upstream state behind `Arc<RwLock<>>` — shared across tasks without a global lock per request
- No `unsafe` — built entirely on tokio/hyper/rustls abstractions
- Config as plain TOML with a documented schema

## Author

Akshit Proothi
