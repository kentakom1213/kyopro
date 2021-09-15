#                 C - IPFL
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc199/tasks/abc199_c

# 解説
# https://atcoder.jp/contests/abc199/editorial/1162
# https://blog.hamayanhamayan.com/entry/2021/04/25/002202

# AC (解説)
# ----------------------------------------

# 方針
# 普通にクエリを処理するとTLE.
# T=1のときは高速、問題はT=2のとき　（らしい）
# 計算量: O(QN) -> 全て T=2のとき

# 反転を高速化する必要がある

# # TLE
# N = int(input())
# S = input()
# Q = int(input())
# queries = [tuple(map(int, input().split())) for _ in range(Q)]

# indexes = list(range(2*N))

# def exchange(a, b):
#     indexes[a], indexes[b] = indexes[b], indexes[a]

# def flip():
#     # 以下の処理が低速
#     for i in range(N):
#         exchange(i, i+N)

# for q, a, b in queries:
#     if q == 1:
#         exchange(int(a)-1, int(b)-1)
#     else:
#         flip()

# print("".join(S[i] for i in indexes))

# 解決策
# 前半、後半を別のリストとして保持

N = int(input())
S = input()
Q = int(input())
queries = [tuple(map(int, input().split())) for _ in range(Q)]

pre, post = list(S[:N]), list(S[N:])

for q, a, b in queries:
    a -= 1
    b -= 1

    if q == 1:
        if b < N:  # 前半同士のスワップ
            pre[a], pre[b] = pre[b], pre[a]
        elif N <= a:  # 後半同士のスワップ
            post[a-N], post[b-N] = post[b-N], post[a-N]
        else:
            pre[a], post[b-N] = post[b-N], pre[a]
    else:
        pre, post = post, pre

print("".join(pre + post))