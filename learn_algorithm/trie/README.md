# Trie

文字列の高速な検索を行えるTrie木の実装をする。

## メソッド

| メソッド名 | 機能 |
| - | - |
| `fn new() -> Trie<V>` | トライ木の初期化 |
| `fn contains_key(&self, key: &str) -> bool` | トライにキーが存在すれば`true`を返す |
| `fn get(&self, key: &str) -> Option<&V>` | キーに対応する値の参照を返す |
| `fn insert(&mut self, key: &str, value: V)` | トライにキーと値を追加する |
| `fn remove(&mut self, key: &str) -> Option<V>` | トライからキーと値を削除する（削除した値を返す） |
| `fn foreach(&self, func: &Fn(&str, &V) -> ())` | トライを巡回する |
| `fn common_prefix(&self, seq: &str, func: &Fn(&str, &V) -> ())` | 共通接頭辞の探索 |
| `fn len(&self) -> usize` | トライの要素数を返す |
| `fn is_empty(&self) -> bool` | トライが空であれば`true` |
| `fn clear(&mut self)` | トライを空にする |


## 出典

> お気楽 Rust プログラミング超入門, Makoto Hiroi, 2018
>
> http://www.nct9.ne.jp/m_hiroi/linux/rust13.html
