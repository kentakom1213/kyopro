# https://atcoder.jp/contests/abc159/tasks/abc159_c

L = int(input())

ans = 0
for i in range(1, L):
    for j in range(i, L-i):
        k = L - (i+j)
        ans = max(ans, i*j*k)

print(ans)
