
N = int(input())
P = list(map(int, input().split()))

# `P[i] > P[i+1]`である最大の`i`
x = 0
for i in range(N-1):
    if P[i] > P[i+1]:
        x = i

# `x`を超えない最小の要素、それを除いた残りの配列
next_top = 0
for i in range(x, N):
    if P[i] < P[x]:
        next_top = P[i]

rem = []
for i in range(x, N):
    if P[i] != next_top:
        rem.append(P[i])

rem.sort(reverse=True)

# 繋げて表示
print(*P[:x], next_top, *rem)
