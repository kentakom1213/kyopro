# B - Same Name
# AC


def mapl(func, iter): return list(map(func, iter))

# input
N = int(input())
name = [tuple(input().split()) for _ in range(N)]

s_name = set(name)

if len(name) == len(s_name):
    print("No")
else:
    print("Yes")
