#             C - Coprime Set             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc118/tasks/arc118_c
# ----------------------------------------

"""comment
- N <= 2500
- N^2までは可能
"""

N = int(input()) - 3

enumed = {0}
ans = [6, 10, 15]

i = 2
while N:
    x = 6 * i
    if x > 10000:
        break
    if x in enumed:
        i += 1
        continue
    ans.append(x)
    enumed.add(x)
    i += 1
    N -= 1

i = 2
while N:
    x = 10 * i
    if x > 10000:
        break
    if x in enumed:
        i += 1
        continue
    ans.append(x)
    enumed.add(x)
    i += 1
    N -= 1

i = 2
while N:
    x = 15 * i
    if x > 10000:
        break
    if x in enumed:
        i += 1
        continue
    ans.append(x)
    enumed.add(x)
    i += 1
    N -= 1

print(*ans)
