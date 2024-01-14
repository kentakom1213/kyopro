N = int(input())

x = bin(N)
ans = 0

for i in range(1, len(x) + 1):
    if x[-i] == '0':
        ans += 1
    else:
        break

print(ans)
