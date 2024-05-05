#             D - Snuke Prime             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc188/tasks/abc188_d
# ----------------------------------------

from bisect import bisect_left

n, prime = map(int, input().split())
a, b, c = [0]*n, [0]*n, [0]*n
for i in range(n):
    a[i], b[i], c[i] = map(int, input().split())
    a[i] -= 1

# 座圧
comp = sorted(set(a)|set(b))

# imos
len_imos = len(comp) + 1
imos = [0] * len_imos
for i in range(n):
    a_idx = bisect_left(comp, a[i])
    b_idx = bisect_left(comp, b[i])
    imos[a_idx] += c[i]
    imos[b_idx] -= c[i]

# 累積和
for i in range(len_imos-1):
    imos[i+1] += imos[i]

# minを取る
ans = 0
for i in range(len(comp)-1):
    ans += min(imos[i], prime) * (comp[i+1] - comp[i])

print(ans)
