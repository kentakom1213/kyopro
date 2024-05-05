# 034 - There are few types of elements（★4）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_ah

# AC
# ----------------------------------------


N, K = map(int, input().split())
A = list(map(int, input().split()))

r = 0
types = 0
nums = {}
res = 0

for l in range(N):
    while (r < N and types <= K):

        if A[r] in nums and nums[A[r]] != 0:
            nums[A[r]] += 1
        else:
            if types > K-1:
                break
            types += 1
            nums[A[r]] = 1
        
        # print("+", types, nums)

        r += 1
    
    if nums[A[l]] == 1:
        types -= 1
    nums[A[l]] -= 1

    res = max(res, r-l)
    # print(A[l:r], types, nums)
    
    if l == r:
        r += 1

print(res)

