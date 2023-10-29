S = input()

ans = -1
for i, c in enumerate(S):
    if c == "a":
        ans = i + 1

print(ans)
