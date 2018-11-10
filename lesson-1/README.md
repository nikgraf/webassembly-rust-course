```sh
brew install rustup
rustup-init
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install wasm-gc
cargo install https
```
