#                B - log2(N)
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc215/tasks/abc215_b

# 盲点

# AC (解説)
# ----------------------------------------

# 10^18 < 2^60

N = int(input())

pow2_x = 1
for i in range(61):
    pow2_x *= 2
    if pow2_x > N:
        print(i)
        break
else:
    print(i)


