#              B - Number Box             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc258/tasks/abc258_b
# ----------------------------------------

N = int(input())
A = [input() for _ in range(N)]

def get_path(start, dir):
    r, c = start
    dr, dc = dir
    res = ""
    for _ in range(N):
        res += A[r][c]
        r = (r+dr) % N
        c = (c+dc) % N
    return int(res)

ans = 0
for i in range(N):
    for j in range(N):
        for dr in range(-1, 2):
            for dc in range(-1, 2):
                if dr == dc == 0:
                    continue
                ans = max(
                    ans,
                    get_path((i, j), (dr, dc))
                )
print(ans)
