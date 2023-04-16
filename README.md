# What's wasm calculator

フロントが react で書かれていて、計算式のパースと評価を wasm が行う電卓。

# requirements

- [cargo-watch](https://crates.io/crates/cargo-watch)
  - `cargo install cargo-watch`
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
  - [install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- node.js

# How to run

```
$ npm run dev
```

npm run dev をすると wasm がコンパイルされてから vite サーバーが立ち上がる。

wasm 部分のテストをしたいときは `wasm-pack test --chrome` を実行してブラウザで 127.0.0.1:8000 にアクセスする。

# How to test

単に rust/src/utils.rs をテストしたい場合:

```
$ cargo test --manifest-path rust/Cargo.toml
```

生成した wasm がブラウザでちゃんと動くかテストしたい場合:

```
$ npm run test
```
