#            D - Floor Function           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc165/tasks/abc165_d
# ----------------------------------------

"""
- mod B での周期性を持つ
"""

A, B, N = map(int, input().split())

ans = (lambda t: (A*t)//B - A*(t//B))(min(B-1, N))

print(ans)