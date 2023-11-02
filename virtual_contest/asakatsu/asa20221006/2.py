# https://atcoder.jp/contests/arc052/tasks/arc052_a

s = input()
ans = ""

for c in s:
    print(c)
    if c in "0123456789":
        ans += c

print(ans)
