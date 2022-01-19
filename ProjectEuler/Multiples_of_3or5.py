#        Multiples of 3 or 5
# ----------------------------------------
# 問題
# https://projecteuler.net/problem=1
# ----------------------------------------

def sum_of_multiple(max_, n):
    max_mul_n = (max_ - 1) // n
    res = (n + max_mul_n * n) * max_mul_n // 2
    return res

if __name__ == "__main__":
    print(sum_of_multiple(10, 3) + sum_of_multiple(10, 5))
    print(sum_of_multiple(1000, 3) + sum_of_multiple(1000, 5) - sum_of_multiple(1000, 15))
