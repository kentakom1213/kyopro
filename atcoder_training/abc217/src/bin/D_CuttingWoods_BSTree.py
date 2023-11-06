#           D - Cutting Woods
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc217/tasks/abc217_d

# TLE
# ----------------------------------------

import sys
def err(*args, **kwargs): print(*args, **kwargs, file=sys.stderr)

class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None
        self.parent = None

class BSTree:
    def __init__(self):
        self.root = None
    
    def insert(self, key):
        """要素の挿入を行う
        Args:
            key : 追加する要素
        """
        if self.root == None:
            self.root = Node(key)

        # 接点(parent)の探索
        parent, result = self.search(key)

        if result: return
        
        # 要素の挿入
        node = Node(key)
        node.parent = parent
        if node.val < parent.val:
            parent.left = node
        else:
            parent.right = node

    def delete(self, key):
        target, result = self.search(key)

        if result:
            self.delete_node(target)
    
    def delete_node(self, node):
        """要素の削除を行う

        Args:
            node (Node): 削除する要素
        """
        if node.left == None:
            self.transplant(node, node.right)
        elif node.right == None:
            self.transplant(node, node.left)
        else:
            y = self.search_min(node.right)

            # yがnodeの直下にない場合、yをnodeの真下に持ってくる必要がある
            if y.parent != node:
                # 一旦yを切り離す
                self.transplant(y, y.right)
                # yの右側にnodeの右側を貼り付ける
                y.right = node.right
                y.right.parent = y

            # yの左側にnodeの左側を貼り付ける
            self.transplant(node, y)
            y.left = node.left
            y.left.parent = y
    
    def transplant(self, u, v):
        """ノードの付け替えを行う

        Args:
            u (Node): 削除されるノード
            v (Node): 子となるノード
        """
        if u.parent == None:
            self.root = v
        elif u.parent.left == u:
            u.parent.left = v
        else:
            u.parent.right = v
        if v != None:
            v.parent = u.parent  # 親の更新
        
    @staticmethod
    def search_min(node) -> Node:
        """木の最小値の探索を行う

        Args:
            node (Node): 部分木のroot
        Returns:
            Node: 木の中で最小の値
        """
        while node.left:
            node = node.left
        
        return node
    
    def search(self, key) -> Node:
        """木の要素を探索する
        
        Args:
            key (Node): 探索する値
        Returns:
            Node: 見つかったノード
        """

        last = None
        node = self.root

        while node:
            if node.val == key:
                return node, True
            
            last = node
            if key < node.val:
                node = node.left
            else:
                node = node.right
        
        return last, False
    
    def lower_bound(self, key):
        node = self.root
        last = None

        while node:
            if key >= node.val:
                last = node
                node = node.right
            else:
                node = node.left
        
        return last.val if last else None
    
    def upper_bound(self, key):
        node = self.root
        last = None

        while node:
            if key < node.val:
                last = node
                node = node.left
            else:
                node = node.right
        
        return last.val if last else None


if __name__ == "__main__":
    L, Q = map(int, input().split())

    tree = BSTree()
    tree.insert(0)
    tree.insert(L)

    for i in range(Q):
        c, x = map(int, input().split())
        if c == 1:
            tree.insert(x)
        else:
            left = tree.lower_bound(x)
            right = tree.upper_bound(x)
            err(left, right)
            print(right - left)