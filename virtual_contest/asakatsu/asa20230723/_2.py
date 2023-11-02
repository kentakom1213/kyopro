# https://atcoder.jp/contests/abc100/tasks/abc100_b

D, N = map(int, input().split())

# 全探索
cnt = 0
for i in range(1, 10_000_000):
    cnt += i % (100 ** D) == 0 and i % (100 ** (D + 1)) != 0
    if cnt == N:
        print(i)
        exit()
