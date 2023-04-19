N = int(input())
S = input()

A = True
B = False
for c in S:
    A &= c != "x"
    B |= c == "o"

print("Yes" if A and B else "No")