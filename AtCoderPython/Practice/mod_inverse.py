#     a ^ (-1) mod p
# <=> ax + py = 1
# を満たす (x. y) を求める。

def modInv(a, p, x, y):
    # 終了条件:
    if p == 0:
        return a, 1, 0




def fib(n):
    if n == 1:
        return 1
    elif n == 2:
        return 1
    else:
        return fib(n - 1) + fib(n - 2)
