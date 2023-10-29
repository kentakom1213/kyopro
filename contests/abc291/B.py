N = int(input())
X = list(map(int, input().split()))

X.sort()

arr = X[N:-N]
ans = sum(arr) / len(arr)
print(ans)
