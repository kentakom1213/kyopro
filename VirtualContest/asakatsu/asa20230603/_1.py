# https://atcoder.jp/contests/abc157/tasks/abc157_b

A = [list(map(int, input().split())) for _ in range(3)]
N = int(input())
B = [int(input()) for _ in range(N)]

check = [[False] * 3 for _ in range(3)]

for b in B:
    for r in range(3):
        for c in range(3):
            if b == A[r][c]:
                check[r][c] = True

# 判定
is_ok = False

# 斜め
is_ok |= check[0][0] and check[1][1] and check[2][2]
is_ok |= check[0][2] and check[1][1] and check[2][0]

# 横
for r in range(3):
    is_ok |= all(check[r])

# 縦
check = list(zip(*check))
for c in range(3):
    is_ok |= all(check[c])


print("Yes" if is_ok else "No")
