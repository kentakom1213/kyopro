
# AC

L = list(map(int, input().split()))
L.sort()

print(2 * L[2] - L[1] - L[0])
