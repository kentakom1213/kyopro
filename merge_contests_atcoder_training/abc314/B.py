from collections import defaultdict

N = int(input())

Cs = []
As = []

for i in range(N):
    Cs.append(int(input()))
    As.append( set(map(int, input().split())) )

X = int(input())

d = defaultdict(list)
d[1e20] = [] 

for i in range(N):
    if X in As[i]:
        d[Cs[i]].append(i + 1)

mini = min(d)

print(len(d[mini]))

if mini == 1e20:
    print()
else:
    print(*d[mini])
