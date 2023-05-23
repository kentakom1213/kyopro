
N = int(input())
S = input()

# 累積和
se, sw = [0]*(N+1), [0]*(N+1)
for i in range(N):
    se[i+1] = se[i]
    sw[i+1] = sw[i]
    if S[i] == "E":
        se[i+1] += 1
    else:
        sw[i+1] += 1

ans = 1e10
for i in range(N):
    # i番目をリーダーに指名
    turn = sw[i] + (se[-1] - se[i+1])
    ans = min(ans, turn)

print(ans)
