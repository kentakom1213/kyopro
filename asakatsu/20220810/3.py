# https://atcoder.jp/contests/arc033/tasks/arc033_2

na, nb = map(int, input().split())
A = set(map(int, input().split()))
B = set(map(int, input().split()))

capAB = A & B
cupAB = A | B

print(len(capAB) / len(cupAB))
