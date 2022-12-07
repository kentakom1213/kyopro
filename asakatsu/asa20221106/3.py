# https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_c

a, b, c = map(int, input().split())

if a+b<=c and 4*a*b < (c-a-b)**2:
    print("Yes")
else:
    print("No")
