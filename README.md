# kyopuro
競プロ用リポジトリ

## 環境
- Python3, PyPy3 (メイン)
- C++ (たまに)

## スニペット
<details><summary>使用しているコードスニペット</summary>
<div>

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

**素因数分解**\
素因数のリストにする
```python
def factoring(n):
    result = [n]
    while result[-1] != 0:
        f = result.pop()
        for i in range(2, int(f ** 0.5 + 1)):
            if f % i == 0:
                result += [i, f//i]
                break
        else:
            result += [f, 0]
    return result[:-1]
```

素因数とその数のタプルのリストにする
```python
def fac_count(n):
    result = []
    for i in range(2, int(n ** 0.5 + 1)):
        if n % i != 0: continue
        counter = 0
        while n % i == 0:
            n //= i
            counter += 1
        result.append((i, counter))
    if n != 1:
        result.append((n, 1))
    return result
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
