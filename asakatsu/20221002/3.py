N = int(input())

ans = 0
fl = [0] * 101010
for _ in range(N):
    a = int(input())
    if fl[a]:
        ans += 1
    else:
        fl[a] = 1
print(ans)