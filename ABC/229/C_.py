
N, W = map(int, input().split())
cheese = [tuple(map(int, input().split())) for _ in range(N)]

cheese.sort()

# 貪欲に取る
res = 0
while W and cheese:
    v, w = cheese.pop()
    if W > w:
        res += v * w
        W -= w
    else:
        res += v * W
        W -= W

print(res)