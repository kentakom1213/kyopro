# https://atcoder.jp/contests/abc013/tasks/abc013_2

a = int(input())
b = int(input())

A = (a - b + 10) % 10
B = (b - a + 10) % 10

print(min(A, B))
