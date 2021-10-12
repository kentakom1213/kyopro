#           D - Grid Coloring
# ----------------------------------------
# 問題
# https:#atcoder.jp/contests/abc069/tasks/arc080_b

# 参考
# https://blog.hamayanhamayan.com/entry/2017/08/06/230026

# AC (解説)
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]
def exprint(x): print(*x, sep="\n")

H, W = map(int, input().split())
N = int(input())
A = list(map(int, input().split()))

res = init_array(W, H)

ptr = 0
for i, a in enumerate(A):
    for _ in range(a):
        res[ptr//H][ptr%H] = i+1
        ptr += 1

# 反転しよう
for i in range(W):
    if i % 2 == 1:
        res[i].reverse()

# 転置しよう
for i in range(H):
    for j in range(W):
        print(res[j][i], end=" ")
    print()
