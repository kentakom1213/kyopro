#            C - Base -2 Number           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc105/tasks/abc105_c
# ----------------------------------------

N = int(input())

def change_base(x, base):
    is_neg = "-" if x < 0 else ""
    x = abs(x)
    res = ""
    while x:
        res += str(x % base)
        x //= base
    return is_neg + res[::-1]

ans = change_base(N, 2)
print(ans)
