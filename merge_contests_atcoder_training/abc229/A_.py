
s1 = input()
s2 = input()

S = s1 + s2

isOK = True

if S.count("#") == 2:
    if s1[0] == "#" and s2[1] == "#" or s1[1] == "#" and s2[0] == "#":
        isOK = False

print("Yes" if isOK else "No")

