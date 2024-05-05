#         C - Synthetic Kadomatsu         
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc119/tasks/abc119_c
# ----------------------------------------

def comb_dub(n, r, arr=[]):
    """重複ありの組合せを生成するジェネレータ"""
    if r == 0:
        yield tuple(arr)
    else:
        for i in range(n):
            yield from comb_dub(n, r-1, arr+[i])

N, A, B, C = map(int, input().split())
L = [int(input()) for _ in range(N)]

# Lの各要素を A,B,C に振り分ける -> 4**8
# 0-> A, 1->B, 2->C, 3->unuse

ans = 1e20

for elems in comb_dub(4, N):
    mp = 0
    a = b = c = 0

    # 合成魔法を使う
    for i, e in enumerate(elems):
        if e == 0:
            if a: mp += 10
            a += L[i]
        elif e == 1:
            if b: mp += 10
            b += L[i]
        elif e == 2:
            if c: mp += 10
            c += L[i]

    if not(a) or not(b) or not(c):
        continue

    # 伸縮魔法を使う
    mp += abs(A - a)
    mp += abs(B - b)
    mp += abs(C - c)

    # 更新
    ans = min(ans, mp)

print(ans)