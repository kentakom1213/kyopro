# ファイル管理システム

- AtCoder, AOJに対応

- ディレクトリ構成
```
kyopuro
  |- abc_training
  |   |- A
  |   |  |_ A_HogeHoge.py
  |   |
  |   |_ B
  |      |_ B_HugaHuga.cpp
  |
  |- arc_training
  |   |_ A
  |      |_ ...
  |
  |- agc_training
  |
  |- AOJ
  |   |_ piyopiyo.py
  |
  |_ Others
      |_ others
         |_ poyapoya.py

```

## 準備
`~/.bashrc`に
```
alias mkfile="/root/miniconda3/bin/python /workspaces/kyopuro/Organizer/make_file.py" 
```
を追加。

## 使い方
1. 解きたい問題のurlをコピー
1. `mkfile (問題のurl) (言語)` コマンドを実行

言語はデフォルトでpython

## 例
```
$mkfile https://atcoder.jp/contests/abc242/tasks/abc242_d cpp
> /workspaces/kyopuro/abc_training/D/D_ABC_Transform.cpp
```
