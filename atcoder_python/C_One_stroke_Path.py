#           C - One-stroke Path
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc054/tasks/abc054_c

# 参考
# https://img.atcoder.jp/abc054/editorial.pdf

# WA
# ----------------------------------------


def mapl(func, iter): return list(map(func, iter))
def filterl(func, iter): return list(filter(func, iter))

# input
N, M = map(int, input().split())
adj = [[] for _ in range(M)]
for _ in range(M):
    a, b = map(int, input().split())
    adj[a - 1].append(b - 1)
    adj[b - 1].append(a - 1)

# solve
counter = 0
def search_graph(adj, rest, depth=0):
    global counter

    print()  # test
    print(adj, rest)  # test

    # when the graph has some isolated vertex
    if any(len(edge) == 0 for edge in adj) and len(rest) > 1:
        return

    if len(rest) == 1:
        # print("  "*depth, rest[0])  # test
        counter += 1
        return

    if 0 in rest:
        print("START")  # test
        for i in rest:
            # print("  "*depth, i)  # test

            adj[i].clear()
            adj = [filterl((lambda k: k != i), v) for v in adj]
            rest = filterl((lambda k: k != i), rest)
            print(f"DEPTH:{depth} POP:{i} REST:{rest}")  # test
            search_graph(adj, rest, depth+1)

    # when the graph has some end vertex
    # elif any(len(edge) == 1 for edge in adj):
    else:
        print("NOT START")  # test
        end = [i for i in range(N) if len(adj[i]) == 1]
        print("END", end)  # test

        if len(end) != 2:
            return

        for i in end:
            # print("  "*depth, i)  # test

            adj[i].clear()
            adj = [filterl((lambda k: k != i), v) for v in adj]
            rest = filterl((lambda k: k != i), rest)
            print(f"DEPTH:{depth} POP:{i} REST:{rest}")  # test
            search_graph(adj, rest, depth+1)


search_graph(adj, list(range(N)))
print(counter)
