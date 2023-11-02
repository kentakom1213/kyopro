# https://atcoder.jp/contests/arc015/tasks/arc015_2

N = int(input())

# 猛暑, 真夏, 夏, 熱帯夜, 冬, 真冬
ans = [0] * 6

for i in range(N):
    M, m = map(float, input().split())

    if M >= 35:
        ans[0] += 1
    if 30 <= M < 35:
        ans[1] += 1
    if 25 <= M < 30:
        ans[2] += 1
    if 25 <= m:
        ans[3] += 1
    if m < 0 and 0 <= M:
        ans[4] += 1
    if M < 0:
        ans[5] += 1
    
print(*ans)