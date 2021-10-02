
# いもす法??
from bisect import bisect_left

N = int(input())
span = [tuple(map(int, input().split())) for _ in range(N)]

S = [a for a, b in span]
G = [a+b for a, b in span]

span_sorted = sorted(set(S + G))

span_comp = [(bisect_left(span_sorted, s), bisect_left(span_sorted, g)) for s, g in zip(S, G)]

print(span_sorted)
print(span_comp)

# いもす法
res = [0] * len(span_sorted)
for s, g in span_comp:
    res[s] += 1
    res[g] -= 1

print(res)

login = [0] * (N+1)
now = 0
for i, diff in enumerate(res):
    now += diff
    if i < N-1:
        login[now] += span_sorted[i+1] - span_sorted[i]

print(login)