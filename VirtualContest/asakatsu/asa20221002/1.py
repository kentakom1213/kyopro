
s = input()
isOK = len(s) % 2 == 0
for i,c in enumerate(s):
    isOK &= c == "hi"[i&2]

print("Yes" if isOK else "No")