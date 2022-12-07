# https://atcoder.jp/contests/abc001/tasks/abc001_2

m = int(input())
mkm = m // 1000

vv = ""

if m < 100:
    vv = "00"
elif 100 <= m <= 5000:
    vv = f"0{m//100}"[-2:]
elif m <= 30000:
    vv = f"{mkm + 50}"
elif m <= 70000:
    vv = f"{(mkm-30)//5 + 80}"
else:
    vv = "89"

print(vv)
