

def count_1(n):
    digit = len(str(n))

    res = 0
    for i in range(1, digit):  # nの1桁前まで
        res += int("1" * i)

    # n桁の検証
    rest = 0
    for i in range(2, digit+1):
        ones_i = int("1"*i + "0"*(digit-i))
        if ones_i < n:
            rest += max(9*10**(digit-i-1), 1) * i
    
    if int(str(n)[0]) > 1: rest += 900
    else:
        rest += (int(str(n)[1]) - 2) * 9 * 
        

    return res, rest


if __name__ == "__main__":
    N = int(input())
    print(count_1(N))

"""

f(0) = 0
f(1) = 1
...
f(10) = 1
f(11) = 2
...
f(19) = 1
f(20) = 0

先頭に並ぶ1の数

1桁 -> 1
2桁 -> 11 = 2, 

n桁の数で上からi桁目までが1
i = n   : 1            * n
i = n-1 : 9            * (n-1)
i = n-2 : 9 * 10       * (n-3)
...
i = 1   : 9 * 10^(i-2) * 1

"""