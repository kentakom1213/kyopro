
X = 5
Y = 5

for i in range(X):
    for j in range(Y):
        l = 2 ** i * j
        r = 2 ** i * (j + 1) - 1
        print(f"{i=}, {j=}, {l=}, {r=}")

        assert j == (l / 2 ** i)
