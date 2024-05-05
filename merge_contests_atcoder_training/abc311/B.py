N, D = map(int, input().split())
S = [input() for _ in range(N)]

ss = [0] * (D + 1)

for i, day in enumerate(zip(*S)):
    isok = True
    for x in day:
        isok &= x == 'o'
    
    if isok:
        ss[i + 1] = ss[i] + 1

print(max(ss))
