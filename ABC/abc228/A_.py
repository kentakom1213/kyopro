# AC

# S, T, X = map(int, input().split())
# S = S or 24
# T = T or 24
# print("Yes" if S <= X < T else "No")

S, T, X = map(int, input().split())

if S < T:
    print("Yes" if S <= X < T else "No")
else:
    if S <= X < 24 or 0 <= X < T:
        print("Yes")
    else:
        print("No")