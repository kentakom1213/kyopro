N, M = map(int, input().split())

P = []
F = []

for _ in range(N):
    p, _, *f = map(int, input().split())
    P.append(p)
    F.append(set(f))

# 全探索
for i in range(N):
    for j in range(N):
        is_ok = P[i] >= P[j] 
        is_ok &= F[j] >= F[i]
        is_ok &= P[i] > P[j] or len(F[j] - F[i]) > 0

        if is_ok:
            # print(i, j, F[i], F[j], F[i] - F[j])
            print("Yes")
            exit()

print("No")
