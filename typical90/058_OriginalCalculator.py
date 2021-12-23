#  058 - Original Calculator（★4）
#  ------------------------------
#  問題
#  https://atcoder.jp/contests/typical90/tasks/typical90_bf
#  ------------------------------

#  MOD = 100000であるから、メモ化によって高々10^5回の計算で済む

n, k = map(int, input().split())

MOD = 100000

def calc(x):
    return (x + sum(map(int, str(x)))) % MOD

memo = [-1] * MOD  # 何回目に現れたかを記録しておく

while k > MOD:
    if memo[n] == -1:
        memo[n] = k
        n = calc(n)
        k -= 1
    else:
        loop = memo[n] - k
        k %= loop

for _ in range(k):
    n = calc(n)

print(n)
