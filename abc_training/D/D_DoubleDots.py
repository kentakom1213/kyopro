#         D - .. (Double Dots)
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc168/tasks/abc168_d

# 根本的に方針が違った
# ----------------------------------------

# union-find
def find(tree, x):
    if tree[x] == x:
        return x
    return find(tree, tree[x])

def union(tree, x, y):
    x = find(tree, x)
    y = find(tree, y)
    tree[y] = x

if __name__ == "__main__":
    N, M = map(int, input().split())

    uf = list(range(N))

    for _ in range(M):
        
        a, b = map(int, input().split())
        a-=1; b-=1
        union(uf, a, b)

        print(uf)
