#             B - Robot Arms
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/keyence2020/tasks/keyence2020_b

# 参考
# https://drken1215.hatenablog.com/entry/2020/01/19/003000
# https://swim-run.net/1193/

# 区間スケジューリング問題というらしい、勉強になる
# AC
# ----------------------------------------

def mapl(func, iter): return list(map(func, iter))

# input
N = int(input())
robots = [ mapl(int, input().split()) for _ in range(N) ]
span = [(x - l, x + l) for x, l in robots]

# selve
span.sort(key=lambda x: x[1])

removed = span[0][0]
count = 0
for start, end in span:
    if removed <= start:
        removed = end
        count += 1

print(count)

### plot
# for start, end in span:
#     s = " " * start + "-" * (end - start)
#     print(s)
