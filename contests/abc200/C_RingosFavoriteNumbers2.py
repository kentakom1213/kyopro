#      C - Ringo's Favorite Numbers 2
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc200/tasks/abc200_c

# AC
# ----------------------------------------

# 方針
# 全探索だと O(n^2) -> 10^10 で間に合わない
# 差が200の倍数 -> 下二桁が一致 && 100の位の偶奇が一致
# 1. (100の位の偶奇, 下二桁) のタプルを作る -> O(n)
# 2. 個数をカウントし、 n*(n-1)/2 を加算   -> O(n)
# -> O(n)

N = int(input())
A = input().split()

def isEven(n): return int(n) % 2 == 0

tuples = {}
for num in A:
    num4 = f"{num:0>4}"
    key = (isEven(num4[-3]), num4[-2:])
    tuples.setdefault(key, 0)
    tuples[key] += 1

res = 0
for val in tuples.values():
    res += val * (val - 1) // 2

print(res)