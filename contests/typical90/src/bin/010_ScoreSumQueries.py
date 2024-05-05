#       010 - Score Sum Queries（★2）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_j

# 累積和頻出だなあ

# AC
# ----------------------------------------

if __name__ == "__main__":
    N = int(input())
    S1, S2 = [0] * (N+1), [0] * (N+1)
    for i in range(1, N+1):
        c, p = map(int, input().split())
        if c == 1:
            S1[i] = S1[i-1] + p
            S2[i] = S2[i-1]
        else:
            S1[i] = S1[i-1]
            S2[i] = S2[i-1] + p
        

    Q = int(input())
    for _ in range(Q):
        l, r = map(int, input().split())
        print(S1[r] - S1[l-1], S2[r] - S2[l-1])