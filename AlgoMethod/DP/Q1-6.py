#               Q1-6. すごろく
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/323

# AC
# ----------------------------------------

N, M = map(int, input().split())
D = list(map(int, input().split()))

DP = [0] * (N + 1)
DP[0] = 1

for i in range(N+1):
    if DP[i] == 1:
        for d in D:
            if i+d <= N:
                DP[i+d] = 1

if DP[N] == 1:
    print("Yes")
else:
    print("No")