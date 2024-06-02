# AC

from bisect import bisect_left

N = int(input())
points = [tuple(map(int, input().split())) for _ in range(N)]

points.sort()

counter = 0
for i in range(N):
    for j in range(N):
        if points[i][0] < points[j][0] and points[i][1] < points[j][1]:
            ul = points[i][0], points[j][1]
            lr = points[j][0], points[i][1]

            is_ul = points[bisect_left(points, ul)] == ul
            is_lr = points[bisect_left(points, lr)] == lr

            if is_ul and is_lr:
                counter += 1


            # if ul in points and lr in points:  # -> ここが遅いか
            #     counter += 1

print(counter)

# TLE

# -> 二分探索にするだけでACでした。
# 今後探索系では絶対使おう