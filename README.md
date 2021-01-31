# ethock - Ethereum Mock

Lightweight Ethereum JSON RPC Mock written in Rust

### Usage

```
ethock_lib = "x.x.x"
```

```rust
fn main() {
    // Serve in blocking mode
    ethock_lib::server::Entry::new("localhost:8545").serve();

    // Serve in non-blocking mode
    ethock_lib::server::Entry::new("localhost:8545").serve_silent();
}
```
