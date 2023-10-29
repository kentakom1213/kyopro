
N = input()

isok = True

for i in range(len(N) - 1):
    isok &= int(N[i]) > int(N[i + 1])

if isok:
    print("Yes")
else:
    print("No")
