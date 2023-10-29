X, Y = map(int, input().split())

if X < Y:
    if abs(X - Y) <= 2:
        print("Yes")
    else:
        print("No")
else:
    if abs(X - Y) <= 3:
        print("Yes")
    else:
        print("No")
