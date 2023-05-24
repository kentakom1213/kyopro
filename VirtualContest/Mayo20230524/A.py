S = input()
n = len(S)
l = S[:n//2]
r = S[n//2+1:]

def is_palin(s):
    return s == s[::-1]

if is_palin(S) and is_palin(l) and is_palin(r):
    print("Yes")
else:
    print("No")
