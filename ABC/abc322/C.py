N, M = map(int, input().split())
A = list(map(int, input().split()))

# 逆順に調べる
ans = [0] * (N + 1)

idx = M - 1
for i in range(N, 0, -1):
    ans[i - 1] = ans[i] + 1
    if A[idx] == i:
        ans[i - 1] = 0
        idx -= 1

print(*ans[:-1], sep="\n")
