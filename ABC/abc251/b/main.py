
N, W = map(int, input().split())
A = list(map(int, input().split()))

ok = [0] * (W + 1)

for i in range(N):
    if A[i] <= W:
        ok[A[i]] = 1

for i in range(N):
    for j in range(i+1, N):
        aa = A[i] + A[j]
        if aa <= W:
            ok[aa] |= 1

for i in range(N):
    for j in range(i+1, N):
        for k in range(j+1, N):
            aaa = A[i] + A[j] + A[k]
            if aaa <= W:
                ok[aaa] |= 1

print(sum(ok))
