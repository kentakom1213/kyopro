N = int(input())

cnt = 0
for i in range(N):
    s = input()
    if s[0] == "F":
        cnt += 1
    
if cnt > N // 2:
    print("Yes")
else:
    print("No")
