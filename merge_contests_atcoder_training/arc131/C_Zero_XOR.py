#               C - Zero XOR              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc131/tasks/arc131_c
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

# 全ての要素のxorをとる
v = 0
for a in A:
    v ^= a


