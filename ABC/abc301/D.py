S = input()
N = int(input())

ans = 0

# セット
d = len(S) - 1
for c in S:
    if c == "1":
        ans |= 1 << d
    d -= 1

# 上の桁から決定していく
d = len(S) - 1
for c in S:
    if c == "?":
        if (ans | 1 << d) <= N:
            ans |= 1 << d
    d -= 1

if ans > N:
    print("-1")
else:
    print(ans)
