
from itertools import combinations

H1, W1 = map(int, input().split())
A = [list(map(int, input().split())) for _ in range(H1)]

H2, W2 = map(int, input().split())
B = [list(map(int, input().split())) for _ in range(H2)]

# 制約的に全探索が可能
# 行列の組合せを全て調べる．

isOK = False
for rows in combinations(range(H1), H2):
    for cols in combinations(range(W1), W2):
        this_combi = True
        for i in range(H2):
            for j in range(W2):
                this_combi &= A[rows[i]][cols[j]] == B[i][j]
        isOK |= this_combi

print("Yes" if isOK else "No")
