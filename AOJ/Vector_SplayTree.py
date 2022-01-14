# SplayTreeで解く

# node: [left, right, parent, size, value]

import sys
def err(*args, **kwargs): print(*args, **kwargs, file=sys.stderr)

def rotate(node):
    p = node[2]
    pp = p[2]

    if p[0] == node:
        c = node[1]
        node[1] = p
        p[0] = c
    else:
        c = node[0]
        node[0] = p
        p[1] = c

    if pp and pp[0] == p:
        pp[0] = node
    if pp and pp[1] == p:
        pp[1] = node
    
    node[2] = pp
    p[2] = node
    if c:
        c[2] = p
    
    # 下から順にupdate
    update(p)
    update(node)

def state(node):
    if node[2] == None:
        return 0
    if node[2][0] == node:
        return 1
    if node[2][1] == node:
        return -1
    return 0

def splay(node):
    while state(node) != 0:
        if state(node[2]) == 0:
            rotate(node)
        elif state(node) == state(node[2]):
            rotate(node[2])
            rotate(node)
        else:
            rotate(node)
            rotate(node)

def update(node):
    node[3] = 1
    if node[0]:
        node[3] += node[0][3]
    if node[1]:
        node[3] += node[1][3]


def get(ind, root):
    while True:
        lsize = root[0][3] if root[0] else 0
        if ind < lsize:
            root = root[0]
        if ind == lsize:
            splay(root)
            return root
        if ind > lsize:
            root = root[1]
            ind -= lsize + 1


if __name__ == "__main__":
    Q = int(input())
    queries = [tuple(map(int, input().split())) for _ in range(Q)]

    vecsize = 0

    # ノード作成
    nodes = [[None, None, None, 1, None] for _ in range(200100)]

    # ノードをつなげる
    for i in range(Q):
        nodes[i][2] = nodes[i+1]
        nodes[i+1][0] = nodes[i]
        update(nodes[i+1])

    root = nodes[Q]
    err(*nodes[:10], sep="\n")

    for q in queries:
        if q[0] == 0:
            root = get(vecsize, root)
            vecsize += 1
            root[4] = q[1]
        elif q[0] == 1:
            root = get(q[1], root)
            print(root[4])
        else:
            vecsize -= 1
