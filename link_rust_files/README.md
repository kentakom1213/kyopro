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
