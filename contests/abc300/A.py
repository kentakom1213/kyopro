N, A, B = map(int, input().split())
C = list(map(int, input().split()))

for i, c in enumerate(C):
    if c == A + B:
        print(i + 1)
        exit()
