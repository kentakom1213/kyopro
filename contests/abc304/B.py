N = input()

d = len(N)

if d <= 3:
    print(N)
else:
    res = N[:3] + "0" * (d - 3)
    print(res)
