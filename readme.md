
# nostr-rs - Notes and Other Stuff Transmitted by Relays (Rust client and relay)

This project implements the `NOSTR` protocol and build a client and relay executable
for use in `NOSTR` social networks. The original `NOSTR` implementation and spec may
be found at https://github.com/fiatjaf/nostr .


# Building

```bash
cargo build --release
```

# Running


```bash
./target/release/nostr-r &
./target/release/nostr-c
```

# Config

The client is configured by a `nostr.toml` file
located at `$HOME/.config/nostr/client.toml`.

An example is [`src/client.toml`](src/client.toml)

The relay is configured by a `nostr.toml` file
located at `$HOME/.config/nostr/relay.toml`.

An example is [`src/relay.toml`](src/relay.toml)

