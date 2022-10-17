
x, k = map(int, input().split())

for i in range(k):
    i10 = 10 ** (i + 1)
    x = (x + 5 * i10 / 10) // i10 * i10

print(int(x))

