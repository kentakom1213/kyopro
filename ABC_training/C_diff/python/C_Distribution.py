#             C - Distribution
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc214/tasks/abc214_c

# 参考
# https://atcoder.jp/contests/abc214/editorial/2438

# 本当に悔しい。DP練習してたんだけどなあ

# AC (解説)
# ----------------------------------------

def exprint(x): print(*x, sep="\n")

N = int(input())
S = list(map(int, input().split()))
T = list(map(int, input().split()))

# engaged = [sum(S[:i]) for i in range(1, N+1)]

# DP = T[:]

# for i in range(N):
#     DP[(i+1)%N] = min(
#         DP[(i+1)%N],
#         
#     )


# print(DP)

### 解説
for i in range(N*2):
    T[(i+1)%N] = min(
        T[(i+1)%N],
        T[i%N] + S[i%N]
    )

exprint(T)

### 意味
# for i in range(2N):
#     DP[次の人] = min(
#         DP[現状],
#         DP[前の人] + S[i]
#     )

### なぜ2N回ループするのか
# 1. ループ1周目
#    時刻(Ti)と、前の人がもらう時間 + Si の小さい方を採用
# 2. ループ二周目
#    ループ1周目の小さい方と、もらった後に回ってくるまでの時間を比較
# 3. ループ3周目
#    2周目の時点で既に一周の比較は済んでいるため、不要