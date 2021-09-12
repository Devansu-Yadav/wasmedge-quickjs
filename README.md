# A wasi quickjs binding for rust
embed js

# Simple demo

See `src/main.rs` and `example_js/demo.js`

## Build

```shell
$ cargo build --target wasm32-wasi --release
```

## Run

```shell
$ wasmedge --dir .:. target/wasm32-wasi/release/quickjs-rs-wasi.wasm
```

# Http demo

Change the include statement in `src/main.rs` to embed `example_js/http_demo.js`

## Build

```shell
$ cargo build --target wasm32-wasi --release
```

## Run

Note: You need to run this from the `example_js` directory since the embedded `http_demo.js` imports an `add.js` from its local directory.

```shell
$ cd example_js
$ wasmedge --dir .:. ../target/wasm32-wasi/release/quickjs-rs-wasi.wasm
```

# Tensorflow demo

Change the include statement in `src/main.rs` to embed `example_js/tensorflow_lite_demo/main.js`

## Build

```shell
$ cargo build --target wasm32-wasi --release --features=tensorflow
```

## Run

```shell
$ cd example_js/tensorflow_lite_demo
$ wasmedge-tensorflow-lite --dir .:. ../../target/wasm32-wasi/release/quickjs-rs-wasi.wasm
```

# Get static-lib & binding.rs
If you want to build a custom libquickjs.a.

See [[quickjs-wasi]](https://github.com/second-state/quickjs-wasi)
