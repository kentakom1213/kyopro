"""
# 考察
Mに応じて下駄を履かせる
"""

N, M = map(int, input().split())
A = list(map(int, input().split()))
A.sort()

for offset in range(M):
    # 二分探索
    l, r = 0, int(1e20)
    while (r - l) > 1:
        mid = (l+r) // 2
        if offset + A[l] > 0:
            l = mid
        else:
            r = mid
    print(offset + A[l])
