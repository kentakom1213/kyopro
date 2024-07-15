#        Even Fibonacci numbers
# ----------------------------------------
# 問題
# https://projecteuler.net/problem=2
# ----------------------------------------

def fibonacci_generator():
    a, b = 1, 1
    while True:
        yield a
        a, b = b, a+b

if __name__ == "__main__":
    MAX = 4000000
    ans = 0
    for n in fibonacci_generator():
        if n >= MAX:
            break
        if n % 2 == 0:
            ans += n
    
    print(ans)
        
