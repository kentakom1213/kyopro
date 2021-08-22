def exprint(l): print(*l, sep="\n")

from collections import deque

INF = int(1e9)

r, c = map(int,input().split())
sy, sx = map(int,input().split())
sy -= 1
sx -= 1
gy, gx = map(int,input().split())
gy -= 1
gx -= 1

maze = [] # 迷路の文字配列
for i in range(r):
    maze.append(input())

# 各座標への最短距離の配列（INFで初期化）
D = [[INF] * c for _ in range(r)]

# 4方向の移動ベクトル
dx = [1,0,-1,0]
dy = [0,1,0,-1]

# スタートからゴールへの最短距離を求めるbfs
def bfs():
    que = deque()
    que.append([sy,sx])   # スタート地点をキューに入れる
    D[sy][sx] = 0       # スタート地点の距離を0とする

    # キューが空になるまで探索
    while len(que) != 0:
        p = que.popleft()   # 先入れのキューの要素を取り出す
        if p[0] == gy and p[1] == gx: # ゴールなら探索終了
            break
        for i in range(4):  # pから4方向に移動 移動後の点は(nx,ny)
            ny = p[0] + dy[i]
            nx = p[1] + dx[i]
            # 移動可能か、訪れたことがあるかの判定
            if 0 <= nx < c and 0 <= ny < r and maze[ny][nx] != "#" and D[ny][nx] == INF:
                # 移動可能ならキューに入れて、その点の距離をpからの距離+1で確定する
                que.append([ny,nx])
                D[ny][nx] = D[p[0]][p[1]] + 1

                # print([ny, nx])  # test
                # exprint(D)  # test
                print(que)  # test
    return D[gy][gx]

print(bfs())
