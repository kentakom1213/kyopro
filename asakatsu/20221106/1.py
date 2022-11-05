# https://atcoder.jp/contests/abc166/tasks/abc166_b

N, K = map(int, input().split())
al = set(range(1, N+1))
for _ in range(K):
    input()
    al -= set(map(int, input().split()))
print(len(al))
