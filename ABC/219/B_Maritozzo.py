# AC

S1, S2, S3, T = (input() for _ in range(4))

res = ""
for t in T:
    if t == "1":
        res += S1
    elif t == "2":
        res += S2
    elif t == "3":
        res += S3

print(res)