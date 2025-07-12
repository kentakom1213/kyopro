
from bisect import bisect_left

N, D = map(int, input().split())
LR = [list(map(int, input().split())) for _ in range(N)]
expanded = sum(LR, [])

llr = sorted(LR)
rlr = sorted(LR, key=lambda x: x[1])

print(llr, rlr)

left = LR[0][1]
