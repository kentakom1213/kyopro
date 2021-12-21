#             B - Mongeness
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc224/tasks/abc224_b

# AC
# ----------------------------------------

# 全探索
# O(n^4) : 50^4 = 6250000、ギリ間にあう？

if __name__ == "__main__":
    H, W = map(int, input().split())
    A = [list(map(int, input().split())) for _ in range(H)]

    isOK = True
    for i1 in range(H):
        for i2 in range(i1, H):
            for j1 in range(W):
                for j2 in range(j1, W):
                    isOK &= A[i1][j1] + A[i2][j2] <= A[i2][j1] + A[i1][j2]

    print("Yes" if isOK else "No")