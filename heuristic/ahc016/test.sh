#!/bin/zsh

## テスト用スクリプト
## `ahc016/`ディレクトリで実行する

binary="../subs/target/debug/$1"

cd ./tools

echo "Score..."

# テストを実行
i=0
sum=0
for input in `ls ./in/*`
do
    local out="./out/$input:t"
    local score=$(cargo run --release --bin tester $binary < $input > /dev/null |& awk 'NR%6==0 {print $3}')
    sum=$((sum + score))
    i=$((i + 1))
    echo "$i: now=$sum"
done

echo "total_score: $sum"
