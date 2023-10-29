
L, R = map(int, input().split())
L -= 1
# R -= 1
S = input()

res = S[:L] + S[L:R][::-1] + S[R:]

print(res)