# https://atcoder.jp/contests/abc097/tasks/abc097_b

X = int(input())

ans = 1
for b in range(2, 1001):
    for p in range(2, 10):
        a = b ** p
        if a <= X and ans < a:
            ans = a

print(ans)
