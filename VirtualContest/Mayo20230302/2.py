a, b, c, x = [int(input()) for _ in range(4)]

ans = 0
for i in range(a+1):
    for j in range(b+1):
        for k in range(c+1):
            ans += 500*i + 100*j + 50*k == x

print(ans)
