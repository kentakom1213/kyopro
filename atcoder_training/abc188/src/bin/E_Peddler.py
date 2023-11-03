#               E - Peddler               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc188/tasks/abc188_e
# ----------------------------------------

"""comment
## 方針
- 木の各頂点に関するdp

## 解法
- dp[j] := 頂点jに到達できる街における金の最安値
- Ai - dp[i] : の最大値が答えとなる
"""

INF = 1e10

def main():
    N, M = map(int, input().split())
    A = list(map(int, input().split()))

    G = [[] for _ in range(N)]

    for _ in range(M):
        x, y = map(int, input().split())
        x-=1; y-=1
        G[x].append(y)

    # dpで解く
    dp = [INF] * N  # dp[j] := 頂点jに到達できる街における金の最安値
    for i in range(N):
        for j in G[i]:
            dp[j] = min(
                dp[j],
                dp[i],
                A[i]
            )
    
    print(max((A[i]-dp[i] for i in range(N))))

if __name__ == "__main__":
    main()
