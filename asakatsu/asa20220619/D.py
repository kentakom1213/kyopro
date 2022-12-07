# https://atcoder.jp/contests/abc109/tasks/abc109_d

H, W = map(int, input().split())
F = [list(map(int, input().split())) for _ in range(H)]

ans = []

# 貪欲に処理
for i in range(H):
    for j in range(W):
        if j+1 < W:
            if F[i][j] & 1:
                F[i][j] -= 1
                F[i][j+1] += 1
                ans.append((i, j, i, j+1))
        if i+1 < H and j == W-1:
            if F[i][j] & 1:
                F[i][j] -= 1
                F[i+1][j] += 1
                ans.append((i, j, i+1, j))

print(len(ans))
for r in ans:
    print(*(v+1 for v in r))
