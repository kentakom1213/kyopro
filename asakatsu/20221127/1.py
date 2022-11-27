# https://atcoder.jp/contests/abc214/tasks/abc214_b

S, T = map(int, input().split())

ans = 0
for a in range(S+1):
    for b in range(S+1):
        for c in range(S+1):
            isOK = a + b + c <= S and a * b * c <= T
            ans += isOK
print(ans)
