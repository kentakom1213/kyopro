#             A - みんなでワイワイみかん             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc056/tasks/arc056_a
# ----------------------------------------

# 貪欲

A, B, K, L = map(int, input().split())
ans = 1e20

# セットをKを超えるまで買うとき
tmp = B * ((K+L-1) // L)

if tmp < ans:
    ans = tmp

# セットをKを超えないように買って、残りをバラで買うとき
tmp = B * (K // L) + A * (K % L)

if tmp < ans:
    ans = tmp

print(ans)
