#             D - Play Train
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc225/tasks/abc225_d

# 本番でできたら完璧だったねえ...
# ----------------------------------------

if __name__ == "__main__":
    N, Q = map(int, input().split())
    queries = [tuple(map(int, input().split())) for _ in range(Q)]

    # trains := [prev, next]
    trains = [[i, i] for i in range(N)]

    for query in queries:
        n, *q = query

        if n == 1:
            x, y = q[0]-1, q[1]-1
            trains[x][1] = y
            trains[y][0] = x

        elif n == 2:
            x, y = q[0]-1, q[1]-1
            trains[x][1] = x
            trains[y][0] = y

        else:
            ptr = q[0]-1

            # 先頭を辿っていく
            while trains[ptr][0] != ptr:
                ptr = trains[ptr][0]

            path = [str(ptr+1)]
            while trains[ptr][1] != ptr:
                ptr = trains[ptr][1]
                path.append(str(ptr+1))
            
            print(len(path), " ".join(path))