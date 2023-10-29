# C - Many Balls
# AC

# input
N = int(input())

magic = []
while N:
    if N % 2 == 0:
        N //= 2
        magic.append("B")
    else:
        N -= 1
        magic.append("A")

magic.reverse()

print("".join(magic))

