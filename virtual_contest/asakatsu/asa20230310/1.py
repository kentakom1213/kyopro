s = input()

isOK = True

isLower = all(c.islower() for c in s)
isUpper = all(c.isupper() for c in s)

isOK &= (not isLower) and (not isUpper)

isOK &= len(s) == len(set(s))

print("Yes" if isOK else "No")
