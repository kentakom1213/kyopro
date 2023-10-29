#              B - アメーバ
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc041/tasks/arc041_b

# AC
# ----------------------------------------

# 順番が関係なさそうなので貪欲に取っていく

def exprint(arr):
    for row in arr:
        print("".join(map(str, row)), end="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N, M = map(int, input().split())
B = [list(map(int, list(input()))) for _ in range(N)]
A = init_array(N, M)

for i in range(1, N-1):
    for j in range(1, M-1):
        num = min(
            B[i-1][j],
            B[i+1][j],
            B[i][j-1],
            B[i][j+1]
        )

        B[i-1][j] -= num
        B[i+1][j] -= num
        B[i][j-1] -= num
        B[i][j+1] -= num

        A[i][j] += num

exprint(A)
