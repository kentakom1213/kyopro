N = int(input())

S = []
mini = 1e20
start = 0

for i in range(N):
    s, a = input().split()
    a = int(a)
    S.append(s)
    if mini > a:
        mini = a
        start = i

for i in range(N):
    print(S[(i + start) % N])
