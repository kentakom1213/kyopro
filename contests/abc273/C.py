

N = int(input())
A = list(map(int, input().split()))

A.sort()

# ランレングス圧縮
comp = []
cnt = 0
cur = A[0]
for a in A:
    if a == cur:
        cnt += 1
    else:
        comp.append(cnt)
        cnt = 1
        cur = a
comp.append(cnt)

l = len(comp)

# 後ろから見ていく
for i in range(1, N+1):
    if i <= l:
        print(comp[-i])
    else:
        print(0)