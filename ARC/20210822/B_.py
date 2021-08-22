# B - Squares
# これは無理です

# input
N = int(input())

# solve
counter = 0

for x in range(1, N):
    for k in range(x):

        y = (x - k) * (x + k)
        print(x, k, y)
        if y <= N:
            counter += 1

print(counter)
