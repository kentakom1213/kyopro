# A - Dial Up
# できない

# input
N, M = map(int, input().split())
S = list(map(int, input().split()))
T = list(map(int, input().split()))
S_rv = list(reversed(S))

# solve
def solver():
    counter = 0
    for t in T:
        if t in S:
            s, s_rv = S.index(t), S_rv.index(t) + 1
            counter += s if s > s_rv else s_rv
            counter += 1
        else:
            return False
    return counter

print(solver())
