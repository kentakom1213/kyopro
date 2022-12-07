# link_rust_files

`rust-analyzer`を有効化するため、Rustのファイルをリンクさせる。

## 方法

rootの`lib.rs`に全てのファイルの情報を記録する

lib.rsの形式

```rust
mod abc_training {
    mod A {
        mod A_A_C;
        mod A_ABC_Preparation;
    }
}

mod dir1 {
    mod inner_dir1 {
        mod file1.rs;
        mod file2.rs;
    }
}
```

## TODO
- [ ] `iterator`で実装

## 参考
- [Rustのmodule完全に理解した。 - zenn](https://zenn.dev/newgyu/articles/3b4677b4086768)
- [Rust の勉強: ファイル一覧](https://makandat.wordpress.com/2022/02/08/rust-%E3%81%AE%E5%8B%89%E5%BC%B7-%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E4%B8%80%E8%A6%A7/)
- [Rust でフォルダ内にあるすべてのファイル名を取得するのが遅い（それほど遅くない）](https://qiita.com/benki/items/70ad2ee44cff9efde778)
