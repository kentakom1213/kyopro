
N = int(input())
S = input()

# 連続する部分のカウント
res = 0

cnt = 0
now = S[0]
for c in S[1:]:
    if now == c:
        cnt += 1
        res += cnt
    else:
        cnt = 0
    now = c

print(res)