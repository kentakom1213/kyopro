#         Q2. 100 までの整数への篩
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/328

# AC
# ----------------------------------------

from itertools import filterfalse

def is_prime(n):
    # if n == 1: return False  # この場合合成数ではないので
    if n == 2: return True
    for i in range(2, int(n ** 0.5) + 2 ):
        if n % i == 0:
            return False
    return True


res = list(
    filterfalse(is_prime,
        filter(lambda n: n == 2 or n % 2 != 0, 
            filter(lambda n: n == 3 or n % 3 != 0,
                filter(lambda n: n == 5 or n % 5 != 0,
                    range(1, 101)
                )
            )
        )
    )
)

print(len(res))