
S = input()

def shiftL(s):
    return s[1:] + s[0]

res = []
for i in range(len(S)):
    res.append(S)
    S = shiftL(S)


res.sort()

print(res[0])
print(res[-1])