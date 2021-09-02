# kyopuro
競プロ用リポジトリ

## 環境
- Python3, PyPy3 (メイン)
- C++ (たまに)

## スニペット
<details><summary>使用しているコードスニペット</summary>
<div>

**input 高速化**
```python
input = sys.stdin.readline
```

**input -> int**
```python
int(input())
```

**input -> int (map)**
```python
map(int, input().split())
```

**input -> int (list)**
```python
list(map(int, input().split()))
```

**initialize array**
```python
def init_array(i, j, val=0):
    return [[val]*j for _ in range(i)]
```

**テストコードの一括コメントアウト**

```re
print(DP)  # test
```
```re
(\s*)(.+# test) -> $1# $2
```
```re
(\s*)# (.+# test) -> $1$2
```

1. コードのテスト部分の最後に `# test` を追加する
2. エディタの置換で `(\s*)(.+# test) -> $1# $2` を実行 (正規表現を有効化する)
3. 戻すときは `(\s*)# (.+# test) -> $1$2`

</div></details>
