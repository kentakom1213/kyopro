#!/bin/zsh

if [[ $(pwd) = $KYOPRO_DIR ]] then
    echo このディレクトリでは操作できません．
    exit 1
fi

# ディレクトリを作成
mkdir -p .vscode

##### Rust #####

# スニペットの元ファイル
TARGET_FILE="$KYOPURO_LIBRARY_DIR/rust.json"

# スニペットのファイル
SNIPPET_FILE='.vscode/rust.code-snippets'

# 既にファイルが存在する場合は削除
unlink $SNIPPET_FILE 2>/dev/null || :

# シンボリックリンクを作成
ln -s $TARGET_FILE $SNIPPET_FILE

echo ".vscode/rust.code-snippets を作成しました．"

##### C++ #####

TARGET_FILES=(
    "tasks.json"
    "c_cpp_properties.json"
)

for target in $TARGET_FILES
do
    # 既にファイルが存在する場合は削除
    unlink ".vscode/$target" 2>/dev/null || :

    # シンボリックリンクを作成
    ln -s "$KYOPRO_DIR/.vscode/$target" ".vscode/$target"

    echo ".vscode/$target を作成しました．"
done

