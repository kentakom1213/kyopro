# https://atcoder.jp/contests/abc136/tasks/abc136_c

# なんかむずくない？

N = int(input())
H = list(map(int, input().split()))

for i in range(N-1, 0, -1):
    if H[i] < H[i-1]:
        H[i-1] -= 1

isOK = True
for i in range(N-1):
    isOK &= H[i] <= H[i+1]

print("Yes" if isOK else "No")
