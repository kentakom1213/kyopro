
t = int(input())

f = lambda x: x**2 + 2*x + 3
res = f(f(f(t) + t) + f(f(t)))

print(res)