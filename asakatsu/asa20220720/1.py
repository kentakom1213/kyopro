# https://atcoder.jp/contests/abc038/tasks/abc038_b

a, b = map(int, input().split())
c, d = map(int, input().split())
  
print("YES" if (a in [c, d] or b in [c, d]) else "NO")