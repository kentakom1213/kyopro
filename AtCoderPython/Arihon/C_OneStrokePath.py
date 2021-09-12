#           C - One-stroke Path
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc054/tasks/abc054_c

# 参考
# https://img.atcoder.jp/abc054/editorial.pdf
# https://qiita.com/greenteabiscuit/items/31c46ab3ceac07e749d2

# AC
# ----------------------------------------

# input
N, M = map(int, input().split())

# adj = [[] for _ in range(M)]  # グラフの隣接リスト -> ダメみたい
# for _ in range(M):
#     a, b = map(int, input().split())
#     adj[a - 1].append(b - 1)
#     adj[b - 1].append(a - 1)

path_matrix = [[False] * N for _ in range(N)]
for _ in range(M):
    a, b = map(int, input().split())
    path_matrix[a - 1][b - 1] = True
    path_matrix[b - 1][a - 1] = True

visited = [False] * N  # 探索済みかどうか

# dfs
def dfs(now):
    # 終了条件J
    if visited[now]:
        return 0

    # visited[now] = True <- こっちに持ってくるとダメ
    if sum(visited) == N - 1:
        return 1

    visited[now] = True
    counter = 0
    for i in range(N):
        if path_matrix[now][i]:
            counter += dfs(i)

    visited[now] = False  # ここなんでFalseに戻すんだろうか

    return counter

print(dfs(0))

### 参考サイトから引用
# def dfs(now, depth):
#     if visited[now]:
#         return 0

#     if depth == N - 1:
#         return 1

#     visited[now] = True
#     total_paths = 0

#     for v in adj[now]:
#         total_paths += dfs(v, depth + 1)

#     visited[now] = False

#     return total_paths

# print(dfs(0, 0))

