#            D - ABC Transform            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc242/tasks/abc242_d

# 割とスッキリとけた
# ----------------------------------------

"""comment
- 逆算していく
"""

import sys
sys.setrecursionlimit(1000000)

S = input()
Q = int(input())
queries = [tuple(map(int, input().split())) for _ in range(Q)]

order = "ABCABC"
next = {
    "A": "BC",
    "B": "CA",
    "C": "AB",
}

def search(t, k):
    """再帰的に探索"""

    if t == 0:
        return S[k]
    if k == 0:
        if t < 3:
            return order[order.index(S[0]) + t]
        return search(t%3, k)
    
    prev = search(t-1, k>>1)
    return next[prev][k&1]

for t, k in queries:
    print(search(t, k-1))
