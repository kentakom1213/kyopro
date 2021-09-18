#               C - Doubled
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc196/tasks/abc196_c

# 解説
# https://atcoder.jp/contests/abc196/editorial/946

# AC (解説)
# ----------------------------------------

# N = int(input())

# # 方針
# # N <= 10^12 であるから桁ごとに考える
# # Nの桁数をdとおくと、2*i <= d を満たすiに関して、
# # 9 * 10^(i-1) を求めれば良い

# N_str = str(N)
# d = len(N_str)

# if d % 2 == 1:
#     doubles_under_d = sum( 9 * 10**(i-1) for i in range(1, d//2 + 1) )
#     print(doubles_under_d)
# else:
#     if N > int("1" + "0" * (d-1)):
#         doubles = int(N_str[0])
#         for c in N_str[1:d//2]:
#             doubles *= int(c) + 1
#     else:
#         doubles = 0
    
#     doubles_under_d = sum( 9 * 10**(i-1) for i in range(1, d//2) )

#     print(doubles + doubles_under_d)

# # WA

# 解説
# 繰り返しは6桁であるため、全探索できる

N = int(input())

d = len(str(N))

max_num = int("1" + "0"*(d//2))

counter = 0
for n in range(1, max_num):
    doubled = int(str(n) + str(n))
    if doubled <= N:
        counter += 1

print(counter)

# -> AC