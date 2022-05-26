# https://atcoder.jp/contests/abc033/tasks/abc033_c

s = input()
spl_add = s.split("+")
spl_mul = [f.split("*") for f in spl_add]
ans = sum(("0" not in f) for f in spl_mul)
print(ans)
