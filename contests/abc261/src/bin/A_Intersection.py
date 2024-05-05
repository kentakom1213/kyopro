#             A - Intersection            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc261/tasks/abc261_a
# ----------------------------------------

a,b,c,d = map(int, input().split())
print(max(0, min(b,d)-max(a,c)))
