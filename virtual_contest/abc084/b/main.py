# B - Postal Code

A, B = map(int, input().split())
S = input()

digits = "0123456789"

isOK = True
for i, c in enumerate(S):
    if i == A:
        isOK &= c == "-"
    else:
        isOK &= c in digits

print("Yes" if isOK else "No")

