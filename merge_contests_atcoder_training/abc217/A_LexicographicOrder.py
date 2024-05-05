# A - Lexicographic Order
# AC

strings = input().split()

if tuple(strings) == tuple(sorted(strings)):
    print("Yes")
else:
    print("No")
