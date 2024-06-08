S = input()

lo = up = 0

for s in S:
    if s.isupper():
        up += 1
    else:
        lo += 1

if lo < up:
    print(S.upper())
else:
    print(S.lower())

