
N = int(input())
A = list(map(int, input().split()))

now = 0
sep = [0]
for x in A:
    now = (now + x) % 360
    print(now)
