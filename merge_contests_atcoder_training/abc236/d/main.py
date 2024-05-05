
# comb(16, 2) = 120

from itertools import combinations

N = int(input())

A = [0] * (2*N - 1)
for i in range(2*N-1):
    a = list(map(int, input().split()))
    A[i] = a


MEMBER = list(range(2*N))

# ペアを選ぶ組合せを列挙
def get_combi(member, combis=[]):

    n = len(member)
    if n == 2:
        yield combis + [tuple(member)]

    for i in range(1, n):
        rest = member[1:i] + member[i+1:]
        res = combis + [(member[0], member[i])]
        yield from get_combi(rest, res)


max_amuse = -1e10
for pairs in get_combi(MEMBER):
    i, j = pairs[0]
    amuse = A[i][j-i-1]
    for i, j in pairs[1:]:
        amuse ^= A[i][j-i-1]
    max_amuse = max(max_amuse, amuse)

print(max_amuse)
