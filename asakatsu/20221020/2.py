# https://atcoder.jp/contests/abc221/tasks/abc221_c

n = input()

ds = sorted(n)

a, b = "0", "0"
while ds:
    top = ds.pop()
    if int(a) == int(b) or int(a) < int(b):
        a += top
    else:
        b += top

print(int(a) * int(b))
