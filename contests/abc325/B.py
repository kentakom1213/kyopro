N = int(input())
W, X = [0] * N, [0] * N
for i in range(N):
    W[i], X[i] = map(int, input().split())

time = [0] * 24

for i in range(N):
    time[X[i]] += W[i]

# 時間帯を全探索
cur = sum(time[:9])
ans = cur
for i in range(24):
    # 更新
    cur -= time[i]
    cur += time[(i + 9) % 24]
    ans = max(ans, cur)

print(ans)
