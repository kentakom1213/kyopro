#        B - Two Colors Card Game
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc091/tasks/abc091_b

# 流石に簡単だった

# AC
# ----------------------------------------

strings = {}

N = int(input())
for _ in range(N):
    s = input()
    if s in strings:
        strings[s] += 1
    else:
        strings[s] = 1

M = int(input())
for _ in range(M):
    t = input()
    if t in strings:
        strings[t] -= 1
    else:
        strings[t] = -1

strings = [val for val in strings.values()]
strings.sort()

print(max(strings[-1], 0))
