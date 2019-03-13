# webassembly-tests-rust

This is a WebAssembly module to provide tests for [webassembly-test-app](https://github.com/ponchofiesta/webassembly-test-app).

## Prerequisites

With these things this project was created and tested:

- Node 10.15 with NPM 6.9 (using NVM)
  ```bash
  # Install NVM and follow instructions
  curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.34.0/install.sh | bash
  # Install Node and NPM
  nvm install 10
  nvm use 10
  npm install npm@6.9.0 -g
  ```
- Rust 1.33
  ```bash
  curl https://sh.rustup.rs -sSf | sh
  ```
- wasm-pack 0.6
  ```bash
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
  ```
- cargo-generate 0.2.2
  ```bash
  cargo install cargo-generate
  ```

## Build

```bash
wasm-pack build
```
