
# https://atcoder.jp/contests/abc148/tasks/abc148_b

N = int(input())
S, T = input().split()
print("".join(s+t for s, t in zip(S, T)))
