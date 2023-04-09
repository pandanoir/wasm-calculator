# What's wasm calculator

フロントが react で書かれていて、計算式のパースと評価を wasm が行う電卓。

# How to run

```
$ npm run dev
```

npm run dev をすると wasm がコンパイルされてから vite サーバーが立ち上がる。

wasm 部分のテストをしたいときは `wasm-pack test --chrome` を実行してブラウザで 127.0.0.1:8000 にアクセスする。
