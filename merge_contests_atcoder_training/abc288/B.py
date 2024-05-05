N, K = map(int, input().split())
S = [input() for _ in range(N)]
print(*sorted(S[:K]))
