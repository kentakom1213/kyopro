
# いもす法??
# from bisect import bisect_left

# N = int(input())
# span = [tuple(map(int, input().split())) for _ in range(N)]

# S = [a for a, b in span]
# G = [a+b for a, b in span]

# span_sorted = sorted(set(S + G))

# span_comp = [(bisect_left(span_sorted, s), bisect_left(span_sorted, g)) for s, g in zip(S, G)]

# print(span_sorted)
# print(span_comp)

# # いもす法
# imos = [0] * len(span_sorted)
# for s, g in span_comp:
#     imos[s] += 1
#     imos[g] -= 1

# print(imos)

# login = [0] * (N+1)
# now = 0
# for i, diff in enumerate(imos):
#     now += diff
#     if i < N-1:
#         login[now] += span_sorted[i+1] - span_sorted[i]

# print(login)


### 頑張って通したい
### AC

from bisect import bisect_left

N = int(input())
span = [tuple(map(int, input().split())) for _ in range(N)]

S = [a for a, b in span]
G = [a+b for a, b in span]

span_sorted = sorted(S + G)

span_comp = [(bisect_left(span_sorted, s), bisect_left(span_sorted, g)) for s, g in zip(S, G)]

# print(span_sorted)
# print(span_comp)

span_len = len(span_sorted)

# いもす法
imos = [0] * span_len
for s, g in span_comp:
    imos[s] += 1
    imos[g] -= 1

# print(imos)

acc = [0] * (span_len + 1)
for i in range(span_len):
    acc[i+1] = acc[i] + imos[i]

acc = acc[1:]
# print(acc)

res = [0] * (N + 1)

for i, v in enumerate(acc):
    if i+1 < span_len:
        res[v] += (span_sorted[i+1] - span_sorted[i])

print(" ".join(map(str, res[1:])))