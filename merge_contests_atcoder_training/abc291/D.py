MOD = 998244353

N = int(input())
AB = [sorted(map(int, input().split())) for _ in range(N)]

ans = 2
prv = AB[0]

for cur in AB[1:]:
    if prv == cur:
        continue
    elif prv[0] != cur[0] \
    and prv[0] != cur[1] \
    and prv[1] != cur[0] \
    and prv[1] != cur[1]:
        # 1つも一致しない
        ans = ans * 2 % MOD
    else:
        # 1つだけ一致
        ans = ans * pow(2, -1, MOD) % MOD
    
    # 更新
    prv = cur

print(ans)


"""
8
877914575 602436426 2
861648772 623690081 2
476190629 262703497 2
971407775 628894325 2
822804784 450968417
         \
161735902 822804784
        | |
161735902 822804784
         X
822804784 161735902
"""
