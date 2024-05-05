#      D - Face Produces Unhappiness
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc140/tasks/abc140_d
# ----------------------------------------

N, K = map(int, input().split())
S = input()

# ランレングス圧縮
comp = []
cur = S[0]
cnt = 0
for c in S:
    if c == cur:
        cnt += 1
    else:
        comp.append(cnt)
        cur = c
        cnt = 1
comp.append(cnt)

# 累積和
lc = len(comp)
S = [0] * (lc+1)
for i in range(lc):
    S[i+1] = S[i] + comp[i]

# 最大化
ans = 0
for l in range(lc):
    r = min(l + 2*K + 1, lc)

    ans = max(ans, S[r]-S[l])
    
# 出力
print(ans-1)
