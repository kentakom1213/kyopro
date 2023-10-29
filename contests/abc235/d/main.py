
# 数字が減ることはあるが、桁が減ることはない
# したがって、桁の数だけ繰り返せばいい


import sys
def err(*args, **kwargs): print(*args, **kwargs, file=sys.stderr)

a, N = map(int, input().split())
MAX_N = N * 10

dp = [1e10] * (MAX_N + 100)
dp[1] = 0

for i, n in enumerate(dp):
    # err(n, i*a, int(str(i)[-1] + str(i)[:-1]))
    if n == 1e10: continue  # 存在しない場合は無視

    # mul
    if i*a <= MAX_N:
        dp[i*a] = min(dp[i*a], n + 1)
        
        # rotate
        if i > 10 and i % 10 != 0:
            rotate = int(str(i)[-1] + str(i)[:-1])
            if rotate <= MAX_N:
                dp[rotate] = min(dp[rotate], n + 1)


print(dp[N] if dp[N] != 1e10 else -1)
