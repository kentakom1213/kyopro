# C - Inverse of Permutation
# AC

N = int(input())
perms = list(map(int, input().split()))

result = [0] * N

for q, p in enumerate(perms):
    result[p - 1] = q + 1

print(" ".join(map(str, result)))
