#                  深さ優先探索                 
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_B&lang=ja
# ----------------------------------------

N = int(input())

G = []
for i in range(N):
    _, _, *adj = (int(v) - 1 for v in input().split())
    G.append(adj)

## 深さ優先探索
stack = [0] # 探索する頂点を保存するリスト
pre = [-1] * N
post = [-1] * N
order = 1

def dfs(u):
    global order
    if pre[u] != -1:
        return

    pre[u] = order
    order += 1

    for v in G[u]:
        if pre[v] == -1:
            dfs(v)

    post[u] = order
    order += 1

# 全ての頂点に繰り返し適用
for i in range(N):
    dfs(i)

for i in range(N):
    print(i + 1, pre[i], post[i])
