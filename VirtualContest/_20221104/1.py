# https://atcoder.jp/contests/abc058/tasks/abc058_a

a, b, c = map(int, input().split())
print("YES" if (c-b) == (b-a) else "NO")
