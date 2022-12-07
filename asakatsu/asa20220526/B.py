
N = int(input())

MAX = 9999999
ans = 0
for i in range(1, MAX):
    x = int(str(i) * 2)
    if x <= N:
        ans += 1

print(ans)