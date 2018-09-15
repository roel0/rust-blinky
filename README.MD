# rust-blinky
-------------

Simple blink example written in rust.

### build
-------------
```
rustup default nightly-2018-09-11
rustup target add thumbv6m-none-eabi
cargo build
```

### Supported devices
-------------

* stm32l0x3

### TODO
-------------

 * Add CI jobs that verify the code can still be compiled after one of the dependencies had an update (the embedded crates in rust still change their API a lot)
 * Add support for more stm32 devices
