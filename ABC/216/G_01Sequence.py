# G - 01Sequence

def mapl(func, iter): return list(map(func, iter))

# input
N, M = map(int, input().split())
restrict = [mapl(int, input().split()) for _ in range(M)]

