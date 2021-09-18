

field = [list(map(int, input().split())) for _ in range(4)]
flatten = sum(field, [])

# 2 ** 16 = 65536 間に合います...

# bit全探索
counter = 0
for i in range(1 << 16):
    flag = True
    for j in range(16):
        # 村を内包するかの判定
        pass


print(counter)