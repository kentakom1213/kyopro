import queue
#               D - Poker
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc193/tasks/abc193_d
# ----------------------------------------

K = int(input())
S, T = input(), input()

cnt = [K] * 10
cntA = [0] * 10
cntB = [0] * 10

for a, b in zip(S, T):
    a = int(a) if a!="#" else 0
    b = int(b) if b!="#" else 0
    cnt[a] -= 1
    cnt[b] -= 1
    cntA[a] += 1
    cntB[b] += 1

# 全探索
ans = 0
for a in range(1, 10):
    for b in range(1, 10):
        # カードが出る確率
        pA = cnt[a]; cnt[a] -= 1
        pB = cnt[b]; cnt[a] += 1

        # 点数を計算
        cntA[a] += 1; cntB[b] += 1
        pointA = sum(i*10**cntA[i] for i in range(1, 10))
        pointB = sum(i*10**cntB[i] for i in range(1, 10))
        cntA[a] -= 1; cntB[b] -= 1

        if pointA > pointB:
            ans += pA * pB

ans /= (9*K - 8) * (9*K - 9)
print(ans)
