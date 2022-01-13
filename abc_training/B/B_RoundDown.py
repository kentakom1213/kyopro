#              B - Round Down
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc196/tasks/abc196_b

# AC
# ----------------------------------------

S = input()

print(S[:S.index(".")] if "." in S else S)