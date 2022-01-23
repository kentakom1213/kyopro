
N = int(input())
A = list(map(int, input().split()))

res = [4] * N
for i in A:
    res[i-1] -= 1

print(res.index(1)+1)
