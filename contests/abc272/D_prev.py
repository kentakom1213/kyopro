from collections import deque

SIGN = [(1, 1), (1, -1), (-1, 1), (-1, -1)]

def get_move(m):
    """ 平方の和がmになる二整数i,jを求める """
    for i in range(-int(-pow(m, 1/2)//1)):
        j = 1
        while i*i + j*j <= m:
            if i*i + j*j == m:
                return (i, j)
            j += 1

    return (0, 0)

def can_reach(r, c):
    return 0 <= r < N and 0 <= c < N

def bfs(start):
    que = deque()
    que.append(start)

    while que:
        cr, cc = que.popleft()
        dr, dc = MOVE
        for sr, sc in SIGN:
            nr = cr + dr * sr
            nc = cc + dc * sc
            if can_reach(nr, nc) and field[nr][nc] == -1:
                field[nr][nc] = field[cr][cc] + 1
                que.append((nr, nc))
    
        # 反転させる
        dc, dr = MOVE
        for sr, sc in SIGN:
            nr = cr + dr * sr
            nc = cc + dc * sc
            if can_reach(nr, nc) and field[nr][nc] == -1:
                field[nr][nc] = field[cr][cc] + 1
                que.append((nr, nc))


if __name__ == "__main__":
    N, M = map(int, input().split())

    field = [[-1]*N for _ in range(N)]
    field[0][0] = 0
    MOVE = get_move(M)

    # 探索
    bfs((0, 0))

    for row in field:
        print(*row)