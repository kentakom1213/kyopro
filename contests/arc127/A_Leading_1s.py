#            A - Leading 1s
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc127/tasks/arc127_a

# 難しい
# ----------------------------------------

# Nが5桁のとき

# f(x) = 5
# -> 11111
# f(x) = 4
# -> 11110 ~ 11119
# f(x) = 3
# -> 11100 ~ 11199
# f(x) = 2
# -> 11000 ~ 11999
# f(x) = 1
# -> 10000 ~ 19999

# 重複は考えず、全ての場合に1を足していけば良い

def main():
    N = int(input())
    D = len(str(N))

    ans = 0
    for d in range(1, D+1):
        for i in range(1, d+1):
            max_ = min(
                int("1"*i + "9"*(d-i)),
                N
            )
            min_ = int("1"*i + "0"*(d-i))
            
            if min_ <= max_:
                # print(min_, max_)
                ans += max_ - min_ + 1
    
    print(ans)

main()
