#!/bin/zsh

# ディレクトリの移動
cd $KYOPRO_DIR/atcoder_training

# ファイルのコピー
if [ -d $1 ]; then
    if [ -f "$1/Cargo.toml" ]; then
        echo "Directory $1 is already exists."
        exit 0
    else
        mv $1 "tmp_$1"
    fi
fi

# ディレクトリの作成
cargo compete new $1

# 戻す
if [ -d "tmp_$1" ]; then
    mv tmp_$1/* $1/src/bin
    rmdir tmp_$1
fi
