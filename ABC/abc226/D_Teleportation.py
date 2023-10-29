
## AC

def gcd(a, b):
    if b == 0: return a
    else:
        return gcd(b, a%b)


N = int(input())
pairs = [tuple(map(int, input().split())) for _ in range(N)]

magic = set()

for i in range(N):
    for j in range(i+1, N):
        dx = pairs[j][0] - pairs[i][0]
        dy = pairs[j][1] - pairs[i][1]
        gcd_xy = gcd(dx, dy)
        move1 = dx // gcd_xy, dy // gcd_xy
        move2 = -move1[0], -move1[1]
        magic.add(move1)  # i -> j
        magic.add(move2)  # j -> i

print(len(magic))

