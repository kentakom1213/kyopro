H, W = map(int, input().split())
ss = [input() for _ in range(H)]
tt = [input() for _ in range(H)]

ssT = sorted(zip(*ss))
ttT = sorted(zip(*tt))

if ssT == ttT:
    print("Yes")
else:
    print("No")
