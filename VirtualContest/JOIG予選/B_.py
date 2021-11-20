
# AC

from string import ascii_lowercase,  ascii_uppercase

N, K = map(int, input().split())
T = input()

res = T[:K-1]
for i in range(K-1, N):
    if T[i] in ascii_uppercase:
        res += ascii_lowercase[ascii_uppercase.index(T[i])]
    else:
        res += ascii_uppercase[ascii_lowercase.index(T[i])]

print(res)