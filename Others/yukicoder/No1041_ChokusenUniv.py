#            No.1041 直線大学
# ----------------------------------------
# 問題
# https://yukicoder.me/problems/no/1041
# ----------------------------------------

# N <= 100 だから3重ループができる

N = int(input())
P = [tuple(map(int, input().split())) for _ in range(N)]

ans = 0
for i in range(N):
    ix, iy = P[i]
    for j in range(i+1, N):
        jx, jy = P[j]
        on_the_line = 2
        for k in range(j+1, N):
            kx, ky = P[k]
            # print(P[i], P[j], P[k], (iy - jy)*(ix - kx) == (iy - ky)*(ix - jx))
            # on_the_line += (iy - jy) / (ix - jx) == (iy - ky) / (iy - kx)
            on_the_line += (iy - jy)*(ix - kx) == (iy - ky)*(ix - jx)
        ans = max(ans, on_the_line)

print(ans)
