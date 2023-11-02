N = int(input())
C = input()

# 交換操作のみを考えれば良い
ans = 0
for i in range(N - 1):
    ans += C[i:i+2] == "WR"

print(ans)
 