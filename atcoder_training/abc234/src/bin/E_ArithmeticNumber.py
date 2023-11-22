#         E - Arithmetic Number
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc234/tasks/abc234_e
#
# AC
# ----------------------------------------

# 10桁以上の等差数は全て同じ数字
# 等差数はそんなに多くないはず
# -> 全探索でいけるかな


X = int(input())

res = 1e20
# 公差 != 0
for d in range(1, 10):
    for i in range(1, 10):
        for j in range(i, 10):
            inc = int( "".join(map(str, range(i, j+1, d))) or 1e20 )
            dec = int( "".join(map(str, range(j, i-2, -d))) or 1e20 )
            
            if X <= inc:
                res = min(res, inc)
            if X <= dec:
                res = min(res, dec)

# 公差 == 0
for digit in range(1, 19):
    for i in range(1, 10):
        num = int(str(i) * digit)
        if X <= num:
            res = min(res, num)

print(res)
