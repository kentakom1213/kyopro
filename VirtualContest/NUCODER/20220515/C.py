
N = int(input())

ans = []
while N > 0:
    if N & 1:
        N -= 1
        ans.append("A")
    else:
        N //= 2
        ans.append("B")

print("".join(reversed(ans)))
