#           A - 119 × 2^23 + 1
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc119/tasks/arc119_a

# ARCの灰diff絶対盛ってるでしょ...

# AC
# ----------------------------------------

# N = a * 2^b + c
# b < 60

# a = N // 2**b
# c = N - a*2^b
# a+b+c = a + b + N - a*2^b

N = int(input())

min_abc = 1e30
for b in range(61):
    a = N // 2**b
    c = N - a * 2**b
    min_abc = min(min_abc, a+b+c)

print(min_abc)
