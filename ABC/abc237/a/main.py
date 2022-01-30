N = int(input())

isOK = - (1 << 31) <= N < (1 << 31)
print("Yes" if isOK else "No")