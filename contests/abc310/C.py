N = int(input())
S = [input() for _ in range(N)]

res = set()

for s in S:
    if s > s[::-1]:
        s = s[::-1]
    res.add(s)

print(len(res))    
