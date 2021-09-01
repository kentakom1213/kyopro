#             D - Islands War
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc103/tasks/abc103_d

# 参考
# https://drken1215.hatenablog.com/entry/2018/07/21/224200

# 隣接するスケジュールを許す区間スケジューリング問題と考えられる。
# AC
# ----------------------------------------

def mapl(func, iter): return list(map(func, iter))

# input
N, M = map(int, input().split())
bridges = [ mapl(int, input().split()) for _ in range(M) ]

end_sorted = sorted(bridges, key=(lambda x: x[1]))

# solve
removed = -1
count = 0
for start, end in end_sorted:
    if removed <= start:
        removed = end
        count += 1

print(count)


### plot
# for start, end in end_sorted:
#     s = " " * start + "-" * (end - start)
#     print(s)
