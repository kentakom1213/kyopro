#                 C - 4/N                 
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/tenka1-2017/tasks/tenka1_2017_c
# ----------------------------------------

"""comment
## 問題
- $4/N = 1/h + 1/n + 1/w$ を満たす正整数 $h, n, w$ を求めよ。
- ただし、 $h, n, w \le 3500$

## 方針
- $3500^2 = 12250000 \simeq 10^7$
- $h, n$ 全探索、$w$ 2分探索

### 式変形
$$
Nhn = w(4hn - Nn - Nh)
$$
"""

N = int(input())
MAX = 3501

# 探索
for h in range(1, MAX):
    for n in range(1, MAX):

        if (4*h*n - N*n - N*h) > 0 and N*h*n % (4*h*n - N*n - N*h) == 0:
            w = N*h*n // (4*h*n - N*n - N*h)
            print(n, h, w)
            exit()

