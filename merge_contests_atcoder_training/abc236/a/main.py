
S = list(input())
a, b = map(int, input().split())

S[a-1], S[b-1] = S[b-1], S[a-1]
print("".join(S))
