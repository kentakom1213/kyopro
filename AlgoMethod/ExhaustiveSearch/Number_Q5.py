#               数字の全探索 5
# ----------------------------------------
# https://algo-method.com/tasks/225
# ----------------------------------------

print(*("FizzBuzz" if i%15==0 else ("Fizz" if i%3==0 else ("Buzz" if i%5==0 else i)) for i in range(1, int(input())+1)) , sep="\n")