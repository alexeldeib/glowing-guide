# glowing-guide

A demonstration of using epoll with pressure stall information.

## Requirements

- [Rust and cargo](https://www.rust-lang.org/tools/install)
- A Linux machine supporting poll/epoll and PSI

## Usage

```bash
cargo run
# or
cargo build
./target/debug/glowing-guide
```

The binary will write a threshold into a pressure interface file.

Use a command like `stress --cpu $(( $(nproc) * 2 ))` to trigger cpu waits.

This should quickly fire an event and exit the binary.
