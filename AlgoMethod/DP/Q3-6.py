#         Q3-6. 部分和問題 (応用 2)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/352

# AC (解説)
# ----------------------------------------

def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N, A, B = map(int, input().split())
nums = list(map(int, input().split()))

dp = init_array(N+1, A, False)
dp[0][0] = True

for i in range(N):
    for j in range(A):
        if dp[i][j]:  # 到達可能な場合
            # 選ぶ場合
            dp[i+1][(j+nums[i])%A] = True

            # 真下に下ろす
            dp[i+1][j] = True


# exprint(dp)
print("Yes" if dp[-1][B] else "No")


# dp[i][j] := N枚目までのカードを使って　x%A = j となる和xを作れるかどうか