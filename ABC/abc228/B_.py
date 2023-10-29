
N, X = map(int, input().split())
A = list(map(int, input().split()))

X -= 1

# 知っている人
know = [False] * N

while not know[X]:
    know[X] = True
    X = A[X] - 1

print(sum(know))