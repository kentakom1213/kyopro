
N = int(input())
A = list(map(int, input().split()))

cnt = [0] * (N+1) # N以上の解はあり得ない

for a in A:
    if a <= N:
        cnt[a] += 1
    else:
        cnt[1] += 1

for i in range(1, N):
    if 
