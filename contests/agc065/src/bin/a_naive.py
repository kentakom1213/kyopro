from itertools import permutations

N, K = map(int, input().split())
A = list(map(int, input().split()))

ans = 0
max_perm = None

for perm in permutations(A):
    tmp = 0
    for i in range(N - 1):
        tmp += (K + perm[i + 1] - perm[i]) % K

    # print(perm, tmp)

    if tmp > ans:
        ans = tmp
        max_perm = perm

print(ans)
print(max_perm)
