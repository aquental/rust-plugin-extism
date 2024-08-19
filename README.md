# rust-plugin-extism

A sample project using [Extism](https://extism.org/) for Rust

## install Extism CLI

```shell
curl -s https://get.extism.org/cli | sh
```

## add rust Wasm target

```shell
rustup target add wasm32-unknown-unknown
```

## inside plugin directory

### build sample

```shell
cargo build --target wasm32-unknown-unknown
```

### run the sample

```shell
extism call target/wasm32-unknown-unknown/debug/rust_pdk_template.wasm greet --input "Benjamin"
```
