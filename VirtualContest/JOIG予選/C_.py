
# AC

N = int(input())
A = list(map(int, input().split()))


# A[i]までに含まれる1の数
cnt1 = [0] * N
cnt0 = [0] * N

cnt1[0] = A[0]
cnt0[0] = 1 - A[0]

for i in range(1, N):
    cnt1[i] = cnt1[i-1] + A[i]
    cnt0[i] = (i+1) - cnt1[i]

res = 1e10
for i in range(N):
    res = min(
        res,
        cnt0[i] + (cnt1[-1] - cnt1[i])
    )

res = min(
    res,
    cnt1[-1]
)

print(res)