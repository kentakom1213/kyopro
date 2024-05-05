# D - Cutting Woods

# from bisect import bisect_left

# L, Q = map(int, input().split())
# queries = [list(map(int, input().split())) for _ in range(Q)]

# bars = [L]
# S_bar = [0, L]

# for c, x in queries:
#     # print(bars)  # test
#     # print(S_bar)  # test

#     where_Sx = bisect_left(S_bar, x)
#     where_x = where_Sx - 1
#     bar_x = bars[where_x]

#     # print(where_x)  # test
#     # print(bar_x)  # test

#     if c == 1:
#         a = x - S_bar[where_x]
#         b = bar_x - a
#         # print(a, b)  # test
#         bars = bars[:where_x] + [a, b] + bars[where_x+1:]
#         S_bar = [sum((bars)[:i]) for i in range(len(bars))]

#     else:
#         print(bar_x)



### 累積和のみで処理
# from bisect import bisect_left

# L, Q = map(int, input().split())
# queries = [map(int, input().split()) for _ in range(Q)]

# S_bar = [0, L]

# for c, x in queries:

#     where_end = bisect_left(S_bar, x)
#     bar_x = S_bar[where_end] - S_bar[where_end - 1]

#     if c == 1:
#         S_bar.insert(where_end, x)

#     else:
#         print(bar_x)


### 高速化
# AC

from bisect import bisect_left
import array

L, Q = map(int, input().split())
queries = [map(int, input().split()) for _ in range(Q)]

S_bar = array.array("I", [0, L])

for c, x in queries:

    where_end = bisect_left(S_bar, x)
    bar_x = S_bar[where_end] - S_bar[where_end - 1]

    if c == 1:
        S_bar.insert(where_end, x)

    else:
        print(bar_x)
