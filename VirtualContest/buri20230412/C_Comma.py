N = input()
d = len(N)

N = int(N)

ans = (d-1) // 3 * (N - pow(10, d-1) + 1)

for i in range(1, d):
    ans += (i-1) // 3 * (pow(10, i) - pow(10, i-1))

print(ans)
