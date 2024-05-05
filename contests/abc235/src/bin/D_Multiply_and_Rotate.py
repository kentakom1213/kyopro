#       D - Multiply and Rotate
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc235/tasks/abc235_d
# ----------------------------------------

# 数字の遷移をグラフの辺として捉えることができる
# 頂点の条件を工夫することで O(NlogN) で解くことができる
# かなり難しかった

import sys
def err(*args, **kwargs): print(*args, **kwargs, file=sys.stderr)
from collections import deque

a, N = map(int, input().split())

# Nの桁数の10倍までを調べれば良い
MAX = 10 ** len(str(N))
is_visited = [-1] * (MAX + 100)
is_visited[1] = 0

# bfs
q = deque()
q.append(1)

while q:
    n = q.popleft()

    mul = n*a
    if n * a <= MAX and is_visited[mul] == -1:
        q.append(mul)
        is_visited[mul] = is_visited[n] + 1
    
    if n > 10 and n % 10 != 0:
        rotate = int( str(n)[-1] + str(n)[:-1] )
        if rotate <= MAX and is_visited[rotate] == -1:
            q.append(rotate)
            is_visited[rotate] = is_visited[n] + 1

print(is_visited[N])
