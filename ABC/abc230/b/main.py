
S = input()

pattern = "oxx"
top = S[0]

if top == "o":
    isOK = True
    for i, s in enumerate(S):
        isOK &= s == pattern[i%3]

else:
    isOK_a = isOK_b = True
    for i, s in enumerate(S):
        isOK_a &= s == pattern[(i+1)%3]

    for i, s in enumerate(S):
        isOK_b &= s == pattern[(i+2)%3]
    
    isOK = isOK_a or isOK_b

print("Yes" if isOK else "No")