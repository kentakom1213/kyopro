
N, M = map(int, input().split())
A = list(map(int, input().split()))

cnt = [0] * N

for a in A:
    cnt[a] += 1

if cnt[a] & 1:
    print(a + 1)
else:
    print(a)
