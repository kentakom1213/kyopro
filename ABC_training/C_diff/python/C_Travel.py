#                C - Travel
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc183/tasks/abc183_c

# AC
# ----------------------------------------

# N <= 8 だから全探索 O(n!)でも N! <= 40320 で間に合うかな

def perm(li):
    if len(li) == 1: yield li
    else:
        for i, el in enumerate(li):
            rest_perm = perm(li[:i] + li[i+1:])
            yield from ([el] + p for p in rest_perm)

def get_length(route):
    lr = len(route)
    route.append(route[0])

    res = 0
    for s, g in zip(route[:lr], route[1:]):
        res += T[s][g]
    
    return res



if __name__ == "__main__":
    N, K = map(int, input().split())
    T = [tuple(map(int, input().split())) for _ in range(N)]

    counter = 0
    for r in perm( list(range(1, N)) ):
        r = [0] + r
        if get_length(r) == K:
            counter += 1
    
    print(counter)