A, M, L, R = map(int, input().split())

L -= A
R -= A

l = (L - 1) // M
r = R // M

print(r - l)
