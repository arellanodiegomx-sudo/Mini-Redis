# Mini Redis

A Redis-compatible key-value store built from scratch in Rust, as a project to learn systems programming — networking, concurrency, and Rust's ownership model.

The server speaks a simplified version of Redis's protocol over TCP, so you can connect to it and send commands directly.

## Features

Currently supported commands:
- `PING` — health check, replies `PONG`
- `SET key value` — store a value under a key
- `GET key` — retrieve a value by key
- `DEL key` — delete a key

Data is stored in memory using a `HashMap` and is lost when the server restarts.

## Running it

Requires [Rust](https://rustup.rs) installed.

```bash
cargo run
```

The server starts listening on `127.0.0.1:6379` (the default Redis port).

## Testing it

In a separate terminal, connect with `netcat` or `redis-cli`:

```bash
nc localhost 6379
```

Example session:
```
SET name sam
+OK
GET name
+sam
DEL name
+OK
GET name
$-1
```

## What's next

This project is being built incrementally. Planned additions:
- Concurrency support (handling multiple clients at once with Tokio)
- Key expiry (`SET key value EX 10`)
- Proper RESP protocol parsing (currently uses simplified plain-text parsing)
- Persistence to disk
- Tests and benchmarks

## Why this project

Built to get hands-on experience with Rust — ownership and borrowing, concurrent programming with `Arc`/`Mutex`, and basic networking — while building something with a clear, well-known reference behavior (real Redis) to test against.