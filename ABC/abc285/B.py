N = int(input())
S = input()

for i in range(1, N):
    cnt = 0
    is_ok = True
    for j in range(N - i):
        is_ok &= S[j] != S[i+j]
        cnt += is_ok
    print(cnt)
