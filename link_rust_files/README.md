# link_rust_files

`rust-analyzer`を有効化するため、Rustのファイルをリンクさせる。

## ディレクトリ構造

```
kyopuro
  ├ .vscode
  │  └ rust-project.json  # ルートを指定
  ├ abc_training
  │  ├ A
  │  │ ├ A_ABC_Preparation.rs
  │  │ ...
  │  │ └ mod.rs           # `A`の中のrustファイルをリンクさせる
  │  ...
  │  └ mod.rs             # `abc_training`の中のディレクトリをリンクさせる
  └ mod.rs                # `kyopuro`内のディレクトリをリンクさせる
```

## mod.rsについて
mod.rsがあるディレクトリ直下にあるファイル/ディレクトリのうち、
1. Rustファイル
2. 内部にRustファイルを持つディレクトリのうち、Cargoプロジェクトでないもの

を追加する。形式は以下の通り

```rust
// FILES
mod {Rustファイル1};
mod {Rustファイル2};
...

// DIRS
mod {ディレクトリ1};
mod {ディレクトリ2};
...
```

## 処理の流れ
1. `root`の呼び出し
2. `root`以下のディレクトリを再帰的に探索
   1. ディレクトリのうちRustファイルを含むものに`mod.rs`を追加
   2. `mod.rs`にRustファイルを追加
   3. `mod.rs`にディレクトリを追加
   4. 探索したディレクトリ以下にRustファイルが含まれるかどうかを返す
