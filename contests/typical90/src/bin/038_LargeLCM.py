#          038 - Large LCM（★3）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_al
# ----------------------------------------

# lcm(a, b) = A / gcd(A, B) * B

## ↓コーナーケースにやられた
# import math

# def gcd(a, b):
#     if b == 0:
#         return a    
#     return gcd(b, a%b)

# if __name__ == "__main__":
#     A, B = map(int, input().split())

#     temp = A // gcd(A, B)
#     digit = math.log10(temp) + math.log10(B)

#     if digit > 18:
#         print("Large")
#     else:
#         print(temp * B)


## なぜlogにこだわるのか

def gcd(a, b):
    if b == 0:
        return a    
    return gcd(b, a%b)

if __name__ == "__main__":
    A, B = map(int, input().split())

    lcm = A // gcd(A, B) * B

    if lcm > 1000000000000000000:
        print("Large")
    else:
        print(lcm)