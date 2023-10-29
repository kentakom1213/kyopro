from collections import defaultdict

N, M, H, K = map(int, input().split())
S = input()
items = defaultdict(int)
for _ in range(M):
    x, y = map(int, input().split())
    items[(x, y)] += 1

MOVE = {
    "R": (1, 0),
    "L": (-1, 0),
    "U": (0, 1),
    "D": (0, -1),
}

# 移動
x, y = 0, 0
hp = H

for c in S:
    # 移動
    x += MOVE[c][0]
    y += MOVE[c][1]
    hp -= 1
    if hp < 0:
        print("No")
        exit()
    elif hp < K and items[(x, y)]:
        hp = K
        # アイテムを消費
        items[(x, y)] -= 1

print("Yes")
