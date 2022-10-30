#         Longest Common Subseqence
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_C&lang=jp

# TLE   その後C++でAC
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

def getLCS(x, y):
    lx, ly = len(x), len(y)
    DP = init_array(lx+1, ly+1)

    for i in range(lx):
        for j in range(ly):
            if x[i] == y[j]:
                DP[i+1][j+1] = DP[i][j] + 1
            else:
                DP[i+1][j+1] = max(
                    DP[i+1][j],
                    DP[i][j+1]
                )

    print(*DP, sep="\n")
    return DP[-1][-1]



if __name__ == "__main__":
    N = int(input())
    datas = [[input(), input()] for _ in range(N)]

    for x, y in datas:
        print(getLCS(x, y))