#          A - Multiple Array
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/agc009/tasks/agc009_a

# AC
# ----------------------------------------

# 後ろから辿っていこう

N = int(input())
B = [0] * N
diff = [0] * N
for i in range(N):
    a, b = map(int, input().split())
    B[i] = b
    diff[i] = -(a % -b)

cnt = acc = 0
for i in range(N-1, -1, -1):
    fill = (diff[i] - acc) % B[i]
    cnt += fill
    acc += fill

print(cnt)
