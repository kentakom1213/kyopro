# SplayTreeで解く

class SplayNode:
    """スプレー木のノード
    Attributes:
        left, right, parent (SplayNode): 親と子のノード
        size (int): 部分木のサイズ
        value: 値
    """
    def __init__(self):
        self.left = self.right = None
        self.parent = None
        self.size = 1
        self.value = None
    
    def rotate(self):
        p = self.parent
        pp = p.parent

        if p.left == self:
            c = self.right
            self.right = p
            p.left = c
        else:
            c = self.left
            self.left = p
            p.right = c

        if pp and pp.left == p:
            pp.left = self
        if pp and pp.right == p:
            pp.right = self
        
        self.parent = pp
        p.parent = self
        if c:
            c.parent = p
        
        # 下から順にupdate
        p.update()
        self.update()

    def state(self):
        if self.parent == None:
            return 0
        if self.parent.left == self:
            return 1
        if self.parent.right == self:
            return -1
        return 0

    def splay(self):
        while self.state() != 0:
            if self.parent.state() == 0:
                self.rotate()
            elif self.state() == self.parent.state():
                self.parent.rotate()
                self.rotate()
            else:
                self.rotate()
                self.rotate()

    def update(self):
        self.size = 1
        if self.left:
            self.size += self.left.size
        if self.right:
            self.size += self.right.size


def get(ind, root):
    while True:
        lsize = root.left.size if root.left else 0
        if ind < lsize:
            root = root.left
        if ind == lsize:
            root.splay()
            return root
        if ind > lsize:
            root = root.right
            ind = ind - lsize - 1


if __name__ == "__main__":
    Q = int(input())
    queries = [tuple(map(int, input().split())) for _ in range(Q)]

    vecsize = 0

    # ノード作成
    node = [SplayNode() for _ in range(220000)]

    # ノードをつなげる
    for i in range(Q):
        node[i].parent = node[i+1]
        node[i+1].left = node[i]
        node[i+1].update()

    root = node[Q]

    for q in queries:
        if q[0] == 0:
            root = get(vecsize, root)
            vecsize += 1
            root.value = q[1]
        elif q[0] == 1:
            root = get(q[1], root)
            print(root.value)
        else:
            vecsize -= 1
