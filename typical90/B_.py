
A, B = input().split()

A_rev, B_rev = reversed(A), reversed(B)

isOK = True
for a, b in zip(A_rev, B_rev):
    isOK &= int(a) + int(b) < 10

print("Easy" if isOK else "Hard")