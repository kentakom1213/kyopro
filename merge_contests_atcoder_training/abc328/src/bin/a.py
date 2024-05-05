N, X = map(int, input().split())
S = list(map(int, input().split()))

ans = 0
for s in S:
    if s <= X:
        ans += s


print(ans)
