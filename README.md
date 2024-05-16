# Leptos Login Example

SSRの機能を使わずに、サーバ側とクライアント側を分けた構成になっている。
`api-boundary`のcrateフォルダに共通のJSONデータ型をまとめた。

- Client side = Leptos (WASM)
- Server side = Axum

## はじめに

- [`rustup`](https://www.rust-lang.org/ja/tools/install)を介してRustをインストールしてください。すでにインストール済みの方は `rustup default stable` 及び `rustup update` を実行し最新のstable使ってることを確認しましょう。 
- `rustup target add wasm32-unknown-unknown` を実行し、WASMのターゲットが無いとクライアントサイドコンパイルできない。
- `trunk` が必要。 `cargo install --locked trunk` もしくは `brew install trunk`

## 実行

### Client

クライアントのファイルをサーブし、且つ特定のパスを別ポート(server)に転送してくれる `trunk` を活用する。

```bash
cd client
trunk serve
```

Clientのファイルを編集して保存すれば再コンパイルされ更新される Hot Reloading 対応。

### Server

Hot reloading 非対応

```bash
cd server
cargo run
```

## アクセス

- `http://127.0.0.1:8080/` にアクセスすれば、`trunk`が提供するサービスに接続し、HTML,JS,WASMファイルを落とす。
- その後、バックエンドは`/api`にアクセスしようとするとき、`http://0.0.0.0:3000`をLISTEN中のAPI serverに転送してくれる。
