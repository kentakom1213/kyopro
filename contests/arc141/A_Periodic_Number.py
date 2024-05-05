#           A - Periodic Number           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc141/tasks/arc141_a
# ----------------------------------------

INF = "9" * 20

def solve():
    N_str = input()
    N = int(N_str)
    D = len(N_str)

    ans = 0

    # dは繰り返しの回数
    for d in range(1, D//2+1):
        N_ = int( N_str[:d] * (D//d) )
        if N_ <= N:
            ans = max(ans, N_)
        
        N__ = int( str(int(N_str[:d])-1) * (D//d) )
        if N__ <= N:
            ans = max(ans, N__)
    
    if int("9"*D) <= N:
        ans = max(ans, int("9"*D))
    
    if int("9"*(D-1)) <= N:
        ans = max(ans, int("9"*(D-1)))
    
    print(ans)


if __name__ == "__main__":
    T = int(input())
    for i in range(T):
        solve()