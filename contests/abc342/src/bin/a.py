from collections import Counter

S = input()

cnt = Counter(S)

a, b = 1e20, ''
for k, v in cnt.items():
    if a > v:
        b = k
        a = v

ans = S.index(b) + 1
print(ans)
