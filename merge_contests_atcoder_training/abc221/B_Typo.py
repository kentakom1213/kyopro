# AC

S = input()
T = input()

isOK = True
count = 0
i = 0
while i < len(S):
    if S[i] == T[i]:
        i += 1
    elif count == 0 and i < len(S) - 1:
        isOK = S[i] == T[i+1] and S[i+1] == T[i]
        i += 2
    else:
        isOK = False
        break

print("Yes" if isOK else "No")