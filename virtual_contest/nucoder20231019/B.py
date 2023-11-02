# https://atcoder.jp/contests/abc181/tasks/abc181_e

from bisect import bisect_left, bisect_right

N, M = map(int, input().split())
H = list(map(int, input().split()))
W = list(map(int, input().split()))

H.sort()

W += [-1e20, -1e20, 1e20, 1e20]
W.sort()

# 生徒は，i番目を抜かしてペアを組む
pre = [0]
for i in range(N // 2):
    tmp = pre[i] + (H[2 * i + 1] - H[2 * i])
    pre.append(tmp)

post = [0]
for i in range(N // 2):
    j = N - (2 * i + 1)
    tmp = post[i] + (H[j] - H[j - 1])
    post.append(tmp)
post.reverse()


# i番目と先生がペアを組む
ans = 1e20
for i in range(N):
    stu = H[i]
    tea_l = bisect_left(W, stu)
    tea_r = bisect_right(W, stu)
    pair = min(abs(stu - W[tea_l - 1]), abs(stu - W[tea_l]), abs(stu - W[tea_r + 1]))
    # print(i, H[:i], H[i + 1:], pre[i // 2], post[(i + 1) // 2], pair)
    if i % 2 == 0:
        bef = pre[i // 2]
        aft = post[(i + 1) // 2]
        tmp = bef + aft + pair
        ans = min(ans, tmp)
    else:
        bef = pre[i // 2]
        aft = post[(i + 1) // 2]
        mae = H[i - 1]
        ushiro = H[i + 1]
        tmp = bef + aft + abs(mae - ushiro) + pair
        ans = min(ans, bef + aft + abs(mae - ushiro) + pair)

print(ans)
