
# 並べ替え
# N <= 10^9 だから 9! = 362880

# bit全探索
# N <= 10^9 だから 2^8 <= 256

# 2つの数が近いほど大きい数になる
# とりあえず貪欲法

#

N = input()
nums = sorted(N)

a, b = nums.pop(), nums.pop()
while nums:
    if int(a) > int(b):
        b += nums.pop()
    else:
        a += nums.pop()

print(int(a) * int(b))
