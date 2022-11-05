# https://atcoder.jp/contests/abc210/tasks/abc210_c

from collections import defaultdict

N, K = map(int, input().split())
C = list(map(int, input().split()))

ans = 0
tmp = 0
mp = defaultdict(int)
# 初期化
for i in range(K):
    if mp[C[i]] == 0:
        tmp += 1
    mp[C[i]] += 1

ans = tmp

for d in range(K, N):
    # 区間からdrop
    mp[C[d-K]] -= 1
    if mp[C[d-K]] == 0:
        tmp -= 1
    
    # 区間にpush
    if mp[C[d]] == 0:
        tmp += 1
    mp[C[d]] += 1

    if ans < tmp:
        ans = tmp

print(ans)
