#!/bin/zsh

# スニペットを作成
SNIPPET_FILE="$KYOPURO_LIBRARY_DIR/rust.code-snippets"

# ディレクトリを作成
mkdir -p .vscode

# シンボリックリンクを作成
ln -s $SNIPPET_FILE .vscode/rust.code-snippets
