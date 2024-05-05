#          C - Brute-force Attack         
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc029/tasks/abc029_c
# ----------------------------------------

N = int(input())

def dfs(s):
    if len(s) == N:
        print(s)
        return
    for c in "abc":
        dfs(s+c)

dfs("")
