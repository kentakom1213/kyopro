#!/bin/zsh

# スニペットの元ファイル
TARGET_FILE="$KYOPURO_LIBRARY_DIR/rust.json"

# スニペットのファイル
SNIPPET_FILE='.vscode/rust.code-snippets'

# ディレクトリを作成
mkdir -p .vscode

# 既にファイルが存在する場合は削除
unlink $SNIPPET_FILE 2>/dev/null || :

# シンボリックリンクを作成
ln -s $TARGET_FILE $SNIPPET_FILE

echo ".vscode/rust.code-snippetsを作成しました．"

