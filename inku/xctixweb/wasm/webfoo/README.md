# build

prequsitites

- [rustwasm/wasm-pack: 📦✨ your favorite rust -> wasm workflow tool!](https://github.com/rustwasm/wasm-pack)
- [bytecodealliance/cargo-wasi: A lightweight Cargo subcommand to build Rust code for the `wasm32-wasi` target](https://github.com/bytecodealliance/cargo-wasi)

```
# frontend, output pkg
wasm-pack build --target=web 
# backend, output target/wasm32-wasi/release/
cargo wasi build --release
```