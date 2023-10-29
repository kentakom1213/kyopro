#             B - Hydrate
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc207/tasks/abc207_b
# ----------------------------------------

# div_ceil = lambda a, b: -(-a // b)

# a, b, c, d = map(int, input().split())

# print(
#     max(
#         div_ceil(a, c*d-b),
#         -1
#     ) if c*d != b else -1
# )

## shortest
a,b,c,d=map(int,input().split());print(max(-int(-a//(c*d-b or-1)),-1))
