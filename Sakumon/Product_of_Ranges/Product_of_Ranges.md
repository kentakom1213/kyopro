
# Product of Ranges

## 問題
$1$ 以上の整数 $N$ と、$1$ 以上の整数 $N$ 個からなる数列 $A = \{a_N\}$ が与えられる。

$Q$ 個のクエリ $(l_j, r_j)$ に対して、以下の式により定義される $P_j$ を $998244353$ で割ったあまりを求めよ。

$$
P_j = \prod_{i=l_j}^{r_j} a_i = a_{l_j}\times a_{l_{j+1}}\times \cdots \times a_{r_j}
$$

## 制約
- $1 \le N \le 10^5$
- $1 \le a_i \le 10^9 ~(1\le i\le N)$
- $1 \le Q \le 10^5$
- $1 \le l_j \le r_j \le N ~(1\le j\le Q)$

---

## 入力
入力は以下の形式で標準入力から与えられる。

```
N Q
A_1 A_2 ... A_N
l_1 r_1
l_2 r_2
...
l_N r_N
```

## 出力
$Q$ 個のクエリについて、$P_j$ を $N$ 行で出力せよ。

---

## 入力例1
```
5 3
1 2 3 4 5
1 3
2 4
1 5
```

## 出力例1
```
6
24
120
```

$1$ 個目のクエリでは $1\times 2\times 3 = 6$ を出力します。\
$2$ 個目のクエリでは $2\times 3\times 4 = 24$ を出力します。\
$3$ 個目のクエリでは $1\times 2\times 3\times 4\times 5 = 120$ を出力します。

---

## 入力例2
```
8 3
31415 92653 58979 32384 62643 38327 95028 84197
1 3
2 7
1 8
```

## 出力例2
```
741501342
924207616
583067047
```

あまりを取ることを忘れないようにしてください。
