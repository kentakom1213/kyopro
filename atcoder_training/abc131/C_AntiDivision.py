#           C - Anti-Division
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc131/tasks/abc131_c
# ----------------------------------------

# CまたはDで割り切れるものを列挙し、A~Bの区間の幅から引く

a, b, c, d = map(int, input().split())

def gcd(a, b):
    if b == 0:
        return a
    else:
        return gcd(b, a % b)
lcm = lambda a, b: a // gcd(a, b) * b

cnt_range = lambda mod: b // mod - -(-a // mod) + 1

mod_c = cnt_range(c)
mod_d = cnt_range(d)
mod_cd = cnt_range(lcm(c, d))

res = b - a + 1 - mod_c - mod_d + mod_cd
print(res)
