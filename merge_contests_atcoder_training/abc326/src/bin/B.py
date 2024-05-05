
N = int(input())

for i in range(N, 1000):
    a, b, c = map(int, str(i))

    if a * b == c:
        print(i)
        exit()
