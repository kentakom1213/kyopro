#            B - Perfect String           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc249/tasks/abc249_b
# ----------------------------------------

S = input()

isOK = not(S.islower() or S.isupper()) \
       and len(S) == len(set(S))

print("Yes" if isOK else "No")

