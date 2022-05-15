
from collections import defaultdict

N = int(input())
S = [input() for _ in range(N)]

dd = defaultdict(int)
for s in S:
    dd[s] += 1

ans = "", 0
for s, cnt in dd.items():
    if ans[1] < cnt:
        ans = s, cnt

print(ans[0])