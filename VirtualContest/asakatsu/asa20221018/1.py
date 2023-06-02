# https://atcoder.jp/contests/abc160/tasks/abc160_b

X = int(input())

a = X // 500
X %= 500
b = X // 5

print(1000*a + 5*b)
