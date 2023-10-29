
# comb(16, 2) = 120

from itertools import combinations

N = int(input())


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

cnt = 0
for pairs in get_combi(MEMBER):
    cnt += 1
    print(pairs)

print(cnt)
