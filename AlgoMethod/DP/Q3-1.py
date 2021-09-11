#         Q3-1. 部分和問題 (導入編)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/336

# AC
# ----------------------------------------

def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N, M = map(int, input().split())
A = list(map(int, input().split()))

DP = init_array(N, M)
DP[0][0] = 1

for i in range(N-1):
    for j in range(M):
        DP[i+1][j] += DP[i][j]
        
        if j + A[i] < M:
            DP[i+1][j + A[i]] += DP[i][j]

# exprint(DP)

print( len( list(filter(lambda x: x!=0, DP[-1])) ) )