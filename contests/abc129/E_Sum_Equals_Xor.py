#            E - Sum Equals Xor
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc129/tasks/abc129_e
# ----------------------------------------

MAX = 101010
MOD = 1_000_000_007

L = input()
N = len(L)

# 桁DP
# dp1[i] := i桁目までみて、L以下であることが確定済み
# dp2[i] := i桁目までみて、L以下であることが未確定
dp1 = [0] * MAX
dp2 = [0] * MAX

dp2[0] = 1

for i in range(N):
    # Lのi桁目が0
    if L[i] == '0':
        # 確定済み → (0,0), (0,1), (1,0)
        dp1[i + 1] = 3 * dp1[i] % MOD
        # 未確定 → (0,0)
        dp2[i + 1] = dp2[i]

    # Lのi桁目が1
    else:
        # 確定済み → (0,0), (0,1), (1,0)
        dp1[i + 1] = (
            3 * dp1[i]  # 確定済み: (0,0), (0,1), (1,0) から遷移
            + dp2[i]    # 未確定: (0,0) から遷移
        ) % MOD
        # 未確定 → (0,1), (1,0)
        dp2[i + 1] = 2 * dp2[i] % MOD

ans = (dp1[N] + dp2[N]) % MOD
print(ans)