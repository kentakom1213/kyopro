# kyopuro
競プロ用リポジトリ

## 言語
- Python3, PyPy3 (メイン)
- C++ (練習中)

## 環境
DockerコンテナでC++の環境を用意。
コンテナ内でPython3.7の環境を構築。
詳細は `/kyopuro/.devcontainer/` を参照。

こちらを使用させていただいています。

https://mycode.rip/how-to-prepare-competitive-programming

## スニペット
<details><summary>Python</summary>
<div>

**input -> int**
```python
int(input())
```

**input -> map (int)**
```python
map(int, input().split())
```

**input -> list (list (int))**
```python
[list(map(int, input().split())) for _ in range(N)]
```

**input -> list, list (int)**
```python
A, B = zip( *(map(int, input().split()) for _ in range(M)) )
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

</div></details>

<details><summary>C++</summary>
<div>

**rep**
```cpp
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
```

**C++環境**
```cpp
#include <bits/stdc++.h>
using namespace std;
```

**print_vector**
```cpp
template <typename T>
void print_vector(vector<T>& vec) {
  cout << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cout << vec.at(i) << " ";
    else cout << vec.at(i);
  }
  cout << " ]" << endl;
}
```

**print_array**
```cpp
template <typename T>
void print_array(vector<vector<T>>& array) {
  int H = array.size();
  int W = array.at(0).size();
  
  cout << "{" << endl;
  for (int i = 0; i < H; i++) {

    cout << "  {";
    for (int j = 0; j < W; j++) {
      if (j < W - 1) cout << array.at(i).at(j) << ", ";
      else cout << array.at(i).at(j);
    }
    cout << "}," << endl;
  }
  cout << "}" << endl;
}
```

**factoring**
```cpp
typedef long long ll;
vector<pair<int, int>> factoring_pair(ll n) {
    vector<pair<int, int>> res;
    for (int i = 2; i*i <= n; i++) {
        int count = 0;
        while (n % i == 0) {
            count++;
            n /= i;
        }
        res.push_back({i, count});
    }
    if (n != 1) res.push_back({n, 1});
    return res;
}
```

</div></details>
