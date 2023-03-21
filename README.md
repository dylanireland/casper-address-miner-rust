# Casper Address Miner

Also available in [JavaScript](https://github.com/dylanireland/casper-address-miner) but I figured Rust would execute faster (but never tested it).

Generates Casper Network-compatable account keypairs repeatedly until it finds one that has a certain character repeated a number of times in the beginning of the public key

## Run

First, adjust the target character and desired length in [main.rs](./src/main.rs).
Note that each extra character multiplies the estimated execution time by 16 (public keys are in hexadecimal).

```bash

git clone https://github.com/dylanireland/casper-address-miner-rust
cd casper-address-miner-rust
cargo build
cargo run
```

## Future Work

* Add support for Secp256k1 accounts
* Variable target character and leading consistent character string on account public key.
* Multithreading
