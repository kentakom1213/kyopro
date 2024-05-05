#             C - Skill Up
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc167/tasks/abc167_c

# 実装ミスなく速く解けるようになりたい

# AC
# ----------------------------------------

# 本来なら部分和問題
# N, Mが非常に小さいためbit全探索でやってみる

N, M, X = map(int, input().split())
C = [0] * N
A = [[] for _ in range(N)]
for i in range(N):
    c, *a = map(int, input().split())
    C[i] = c
    A[i] = a

# bit全探索
mini = 1e10
for i in range(1 << N):
    total = 0
    skills = [0] * M

    for j in range(N):
        # print((i >> j) & 1)
        if (i >> j) & 1:
            total += C[j]

            for k in range(M):
                skills[k] += A[j][k]
            # print(skills)

    # print(total, skills)
    if all(skill >= X for skill in skills):
        mini = min(mini, total)

    # print()

print(mini if mini != 1e10 else -1)