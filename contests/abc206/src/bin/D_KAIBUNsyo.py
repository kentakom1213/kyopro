#             D - KAIBUNsyo
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc206/tasks/abc206_d

# 難しい

# AC
# ----------------------------------------

from collections import Counter
N_MAX = 200010

N = int(input())
A = list(map(int, input().split()))

flag = Counter(A)

# 前半, 後半の反転
left = A[:N//2]
right = A[:(N-1)//2:-1]

# グラフに格納
G = [[] for _ in range(N_MAX)]
for l, r in zip(left, right):
    G[l].append(r)
    G[r].append(l)

# 結合を調べる
ans = len(flag)  # 出てくる文字の種類数
for i in range(N_MAX):
    if flag[i]:
        ans -= 1
        
        # dfs
        stack = [i]
        while stack:
            cur = stack.pop()
            flag[cur] = 0
            for nxt in G[cur]:
                if flag[nxt]:
                    stack.append(nxt)

print(ans)
