# ファイル管理システム

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
