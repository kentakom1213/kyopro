# https://atcoder.jp/contests/abc080/tasks/abc080_b

def f(n):
    return sum(map(int, str(n)))

n = int(input())

if n % f(n) == 0:
    print("Yes")
else:
    print("No")
