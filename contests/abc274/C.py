
N = int(input())
A = list(map(int, input().split()))

ans = [0]
for i, a in enumerate(A):
    ans.append(ans[a-1] + 1)
    ans.append(ans[a-1] + 1)

print(*ans)
