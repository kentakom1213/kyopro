#              E - Round Trip             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc276/tasks/abc276_e
# ----------------------------------------

def solve(h, w, field):
    n = len(field)
    s = field.find("S")

    MOVE = (-w-2, -1, 1, w+2)
    visited = [-1] * n

    q = []
    for i, d in enumerate(MOVE):
        if field[s+d] == ".":
            q.append(s+d)
            visited[s+d] = i
    
    while q:
        u = q.pop()
        for d in MOVE:
            if field[u+d] != ".":
                continue
            if visited[u+d] == visited[u]:
                continue
            if visited[u+d] == -1:
                visited[u+d] = visited[u]
                q.append(u+d)
                continue
            return "Yes"

    return "No"

H, W = map(int, input().split())
C = "#"*(W+2) + "##".join(input() for _ in range(H)) + "#"*(W+2)

print(solve(H, W, C))
