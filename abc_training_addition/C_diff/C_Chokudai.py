# C - Chokudai
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc211/tasks/abc211_c

# 参考
# https://atcoder.jp/contests/abc211/editorial/2287

# かなり難しかった、DPに慣れる必要がある

# AC (解説)
# ----------------------------------------

### LCS によるアプローチ
# def exprint(x): print(*x, sep="\n")
# def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

# ceo = "chokudai"
# S = input()

# DP = init_array(len(ceo)+1, len(S)+1)

# counter = 0
# for i in range(len(ceo)):
#     for j in range(len(S)):
#         if ceo[i] == S[j]:
#             counter += 1
#             DP[i+1][j+1] = DP[i][j] + 1
#         else:
#             DP[i+1][j+1] = max(
#                 DP[i][j+1],
#                 DP[i+1][j]
#             )

# exprint(DP)
# print(counter)
# -> だめですね

### 解説
def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

ceo = "chokudai"
S = input()
lc, ls = len(ceo), len(S)

MOD = int(1e9 + 7)
DP = [[1] + [0]*lc for _ in range(ls+1)]

for i in range(1, ls+1):
    for j in range(1, lc+1):

        if S[i-1] == ceo[j-1]:  # ...(i)
            DP[i][j] = ( DP[i-1][j] + DP[i-1][j-1] ) % MOD 
        else:                   # ...(ii)
            DP[i][j] = DP[i-1][j]


print(DP[-1][-1])

### 解釈
# (i)について
#    S[i]に下線を引くことはありえないためDP[i-1][j]の結果を引き継ぐ
# (ii)について
#    - S[i]に下線を引かない場合
#      (i)と同様にDP[i-1][j]に等しい
#    - S[i]に下線を引く場合
#      Sの(i-1)文字目までを使ってTの(i-1)文字目までを選択する方法　の場合の数に等しい
#      よってDP[i-1][j-1]
#    上2つの和である.
