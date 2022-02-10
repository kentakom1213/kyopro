#!/bin/zsh

# 実行
$(dirname `pwd`)/$1/$1 < $2 > ./out.txt

# 出力を得る
cargo run --release --bin vis $2 out.txt

# 表示
open /Applications/Google\ Chrome.app/ out.svg
