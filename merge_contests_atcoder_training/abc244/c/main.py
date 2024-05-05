
N = int(input())

rem = set(range(1, 2*N+2))
print(1)
rem.remove(1)

for _ in range(N):
    x = int(input())
    rem.remove(x)
    print(rem.pop())