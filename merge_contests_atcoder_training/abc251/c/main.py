
from collections import defaultdict

N = int(input())

dd = defaultdict(int)
sub = []
for i in range(N):
    s, t = input().split()
    if not dd[s]:
        sub.append((int(t), -i))
        dd[s] = 1

sub.sort()
print(-sub[-1][1] + 1)
