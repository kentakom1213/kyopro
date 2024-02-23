#!/bin/zsh

# スニペットを作成
SNIPPET_FILE="$KYOPURO_LIBRARY_DIR/rust.json"

# ディレクトリを作成
mkdir -p .vscode

if [[ -e .vscode/rust.code-snippets ]]; then
  # 既にファイルが存在する場合は削除
  rm .vscode/rust.code-snippets
fi

# シンボリックリンクを作成
ln -s $SNIPPET_FILE .vscode/rust.code-snippets
