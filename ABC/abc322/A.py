N = int(input())
S = input()

for i in range(N):
    if S[i:i + 3] == "ABC":
        print(i + 1)
        exit()

print(-1)
