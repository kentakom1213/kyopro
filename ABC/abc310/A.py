N, P, Q = map(int, input().split())
D = list(map(int, input().split()))

ans = min(P, Q + min(D))
print(ans)
