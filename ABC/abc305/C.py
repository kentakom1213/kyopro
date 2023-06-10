H, W = map(int, input().split())
S = [[c == "." for c in input()] for _ in range(H)]
T = list(zip(*S))


# 全て白の行と列を削除していく
b, t = 0, H - 1
l, r = 0, W - 1

while all(S[t]):
        t -= 1

while all(S[b]):
        b += 1

while all(T[l]):
        l += 1

while all(T[r]):
        r -= 1

# 区間内部を探索
xx, yy = -1, -1
for y in range(b, t + 1):
    for x in range(l, r + 1):
           if S[y][x]:
                  xx = x
                  yy = y

print(yy + 1, xx + 1)
