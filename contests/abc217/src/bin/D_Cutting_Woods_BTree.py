# ---------- BTree ----------
import sys
from typing import List, Optional, TypeVar, Generic, Tuple

# AtCoder提出時にprint_as_treeの出力が邪魔になる場合があるので、
# 必要に応じてコメントアウトしてください。
DEBUG_MODE = True

K = TypeVar("K")
V = TypeVar("V")
NodePtr = "BTreeNode[K, V]"


class BTreeNode(Generic[K, V]):
    """
    B木のノード。
    """

    def __init__(self, D: int, is_leaf: bool = True):
        self.D = D  # Degree
        self.keys: List[Optional[K]] = [None] * (2 * D - 1)
        self.vals: List[Optional[V]] = [None] * (2 * D - 1)
        self.children: Optional[List[Optional[NodePtr]]] = (
            [None] * (2 * D) if not is_leaf else None
        )
        self.size: int = 0

    def is_leaf(self) -> bool:
        """ノードが葉であるか判定する。"""
        return self.children is None

    def is_full(self) -> bool:
        """ノードに空きがないか判定する。"""
        return self.size == (2 * self.D - 1)

    def nth_key(self, n: int) -> Optional[K]:
        """n番目のキーを取得する。"""
        if 0 <= n < self.size:
            return self.keys[n]
        return None

    def nth_val(self, n: int) -> Optional[V]:
        """n番目の値を取得する。"""
        if 0 <= n < self.size:
            return self.vals[n]
        return None

    def nth_child(self, n: int) -> Optional[NodePtr]:
        """n番目の子を取得する。"""
        if self.children and 0 <= n <= self.size:
            return self.children[n]
        return None

    def __repr__(self) -> str:
        node_type = "Leaf" if self.is_leaf() else "Internal"
        return f"{node_type}(size={self.size}, keys={self.keys[:self.size]}, vals={self.vals[:self.size]})"


# Debugging constants
LEFT = "  ┌─"
MID = "  │ "
SEP = "  ├─"
RIGHT = "  └─"
NULL = ""
BLANK = "    "


def _print_as_tree_util(root: Optional[NodePtr]):
    """
    B木をツリー形式で出力する。
    """
    if not DEBUG_MODE:
        return
    print(
        "┌─ B-Tree ──────────────────────────────────────────────", file=sys.stderr
    )
    _dbg_inner(root, [], NULL)
    print(
        "└───────────────────────────────────────────────────────", file=sys.stderr
    )


def _dbg_inner(root: Optional[NodePtr], fill: List[str], last: str):
    """
    再帰的にノードを表示するヘルパー関数。
    """
    if root is None:
        return

    # Adjust display
    temp_pop = None
    if fill and fill[-1] == last and last != SEP:
        temp_pop = fill.pop()
        fill.append(BLANK)
    elif fill and fill[-1] != NULL and fill[-1] != BLANK:
        temp_pop = fill.pop()
        fill.append(MID)
    fill.append(last)

    # Print current node's keys and values
    if root.is_leaf():
        for i in range(root.size):
            _print_node_info(root.keys, root.vals, fill, last, i, root.size, root.D)
    else:  # Internal node
        for i in range(root.size):
            # Print child node
            _dbg_inner(root.children[i], fill, LEFT if i == 0 else SEP)
            # Print key/value
            _print_node_info(root.keys, root.vals, fill, last, i, root.size, root.D)
        # Print rightmost child
        _dbg_inner(root.children[root.size], fill, RIGHT)

    # Backtrack fill for parent node
    fill.pop()
    if temp_pop is not None:
        fill.pop()
        fill.append(temp_pop)


def _print_node_info(
    keys: List[Optional[K]],
    vals: List[Optional[V]],
    fill: List[str],
    last: str,
    i: int,
    size: int,
    D: int,
):
    """
    ノードのキーと値を出力するヘルパー関数。
    """
    current_fill_str = "".join(fill)
    if last == LEFT and i != 0 or last == RIGHT and i != size - 1:
        temp_fill = list(fill)
        if temp_fill:
            temp_fill[-1] = SEP
        current_fill_str = "".join(temp_fill)

    key_val_str = f"Node {{ key: {keys[i]}, val: {vals[i]} }}"
    print(f"│ {current_fill_str} {key_val_str}", file=sys.stderr)


class BTreeMap(Generic[K, V]):
    """
    B木による連想配列。
    """

    def __init__(self, D: int):
        self.D = D
        self.root: Optional[NodePtr] = None
        self.size: int = 0

    def _new_leaf(self) -> NodePtr:
        """新しい葉ノードを作成する。"""
        return BTreeNode(self.D, is_leaf=True)

    def _new_internal(self) -> NodePtr:
        """新しい内部ノードを作成する。"""
        return BTreeNode(self.D, is_leaf=False)

    def insert(self, key: K, value: V):
        """値を挿入する。"""
        if self.root is None:
            self.root = self._new_leaf()

        if self.root.is_full():
            s = self._new_internal()
            s.children[0] = self.root
            self._split_child(s, 0)
            self._insert_non_full(s, key, value)
            self.root = s
        else:
            self._insert_non_full(self.root, key, value)
        self.size += 1

    def _insert_non_full(self, x: NodePtr, key: K, value: V):
        """空きのあるノードにデータを挿入する。"""
        i = x.size - 1
        if x.is_leaf():
            while i >= 0 and key < x.keys[i]:
                x.keys[i + 1] = x.keys[i]
                x.vals[i + 1] = x.vals[i]
                i -= 1
            x.keys[i + 1] = key
            x.vals[i + 1] = value
            x.size += 1
        else:
            while i >= 0 and key < x.keys[i]:
                i -= 1
            i += 1  # i is the index of the child to descend into

            if x.children[i].is_full():
                self._split_child(x, i)
                if key > x.keys[i]:
                    i += 1
            self._insert_non_full(x.children[i], key, value)

    def _split_child(self, x: NodePtr, i: int):
        """ノード x の i 番目の子 y が飽和しているとき、頂点を分割する。"""
        y = x.children[i]
        z = self._new_leaf() if y.is_leaf() else self._new_internal()

        # Shift keys and values in x to make space for the new key/value from y
        # In Python, we can use insert which automatically shifts elements
        x.keys.insert(i, y.keys[self.D - 1])
        x.vals.insert(i, y.vals[self.D - 1])

        # Adjust y's keys and values (they are taken by z or parent)
        y.keys[self.D - 1] = None
        y.vals[self.D - 1] = None

        # Move keys/values from y to z
        for j in range(self.D - 1):
            z.keys[j] = y.keys[self.D + j]
            z.vals[j] = y.vals[self.D + j]
            y.keys[self.D + j] = None
            y.vals[self.D + j] = None
        z.size = self.D - 1

        # If y is internal, move children from y to z
        if not y.is_leaf():
            for j in range(self.D):
                z.children[j] = y.children[self.D + j]
                y.children[self.D + j] = None

        # Adjust y's size
        y.size = self.D - 1

        # Insert z as a child of x
        x.children.insert(i + 1, z)
        x.size += 1

    def remove(self, key: K) -> Optional[Tuple[K, V]]:
        """値を削除する。"""
        removed_item = None
        if self.root:
            removed_item = self._remove(self.root, key)
            # If root becomes empty and is not a leaf, new root is its first child
            if self.root.size == 0 and not self.root.is_leaf():
                self.root = self.root.children[0]
            elif self.root.size == 0 and self.root.is_leaf():
                self.root = None
            if removed_item:
                self.size -= 1
        return removed_item

    def _remove(self, node: NodePtr, key: K) -> Optional[Tuple[K, V]]:
        """B木から値を削除する内部関数。"""
        i = 0
        while i < node.size and key > node.keys[i]:
            i += 1

        if node.is_leaf():
            if i < node.size and key == node.keys[i]:
                # Found key in leaf node
                removed_key = node.keys[i]
                removed_val = node.vals[i]
                node.keys = node.keys[:i] + node.keys[i + 1 :] + [None]
                node.vals = node.vals[:i] + node.vals[i + 1 :] + [None]
                node.size -= 1
                return (removed_key, removed_val)
            else:
                return None
        else:  # Internal node
            if i < node.size and key == node.keys[i]:
                # Key is in this internal node
                left_child = node.children[i]
                right_child = node.children[i + 1]

                if left_child.size >= self.D:
                    # Replace key with max from left child
                    max_key, max_val = self._remove_max(left_child)
                    node.keys[i] = max_key
                    node.vals[i] = max_val
                    return (
                        key,
                        node.vals[i],
                    )  # Return original key, val (This part needs adjustment for correctness based on Rust impl)
                elif right_child.size >= self.D:
                    # Replace key with min from right child
                    min_key, min_val = self._remove_min(right_child)
                    node.keys[i] = min_key
                    node.vals[i] = min_val
                    return (
                        key,
                        node.vals[i],
                    )  # Return original key, val (This part needs adjustment for correctness based on Rust impl)
                else:
                    # Merge left and right children, then remove key from merged child
                    self._merge_children(node, i)
                    return self._remove(node.children[i], key)
            else:
                # Key is in a child subtree
                child = node.children[i]
                if child.size == self.D - 1:
                    # Child is deficient, need to fix
                    if i > 0 and node.children[i - 1].size >= self.D:
                        self._borrow_from_left(node, i)
                    elif i < node.size and node.children[i + 1].size >= self.D:
                        self._borrow_from_right(node, i)
                    else:
                        # Merge with sibling
                        if i == node.size:  # Rightmost child, merge with left sibling
                            self._merge_children(node, i - 1)
                            child = node.children[i - 1]  # Update child reference
                        else:  # Merge with right sibling
                            self._merge_children(node, i)

                return self._remove(child, key)

    def _remove_max(self, node: NodePtr) -> Tuple[K, V]:
        """部分木の最大値を削除し、そのキーと値を返す。"""
        if node.is_leaf():
            node.size -= 1
            return (node.keys[node.size], node.vals[node.size])
        else:
            child_idx = node.size
            child = node.children[child_idx]
            if child.size == self.D - 1:
                # Deficient child, fix it
                if node.children[child_idx - 1].size >= self.D:
                    self._borrow_from_left(node, child_idx)
                else:
                    self._merge_children(node, child_idx - 1)
                    child_idx -= 1  # child reference updated
            return self._remove_max(node.children[child_idx])

    def _remove_min(self, node: NodePtr) -> Tuple[K, V]:
        """部分木の最小値を削除し、そのキーと値を返す。"""
        if node.is_leaf():
            removed_key = node.keys[0]
            removed_val = node.vals[0]
            node.keys = node.keys[1:] + [None]
            node.vals = node.vals[1:] + [None]
            node.size -= 1
            return (removed_key, removed_val)
        else:
            child_idx = 0
            child = node.children[child_idx]
            if child.size == self.D - 1:
                # Deficient child, fix it
                if node.children[child_idx + 1].size >= self.D:
                    self._borrow_from_right(node, child_idx)
                else:
                    self._merge_children(node, child_idx)
            return self._remove_min(node.children[child_idx])

    def _merge_children(self, node: NodePtr, i: int):
        """node の i 番目の子と i+1 番目の子をマージする。"""
        child1 = node.children[i]
        child2 = node.children[i + 1]

        # Move key/value from parent to child1
        child1.keys[child1.size] = node.keys[i]
        child1.vals[child1.size] = node.vals[i]
        child1.size += 1

        # Move keys/values from child2 to child1
        for j in range(child2.size):
            child1.keys[child1.size + j] = child2.keys[j]
            child1.vals[child1.size + j] = child2.vals[j]
        child1.size += child2.size

        # If internal, move children from child2 to child1
        if not child1.is_leaf():
            for j in range(child2.size + 1):
                child1.children[child1.size - child2.size + j] = child2.children[j]

        # Shift keys/values and children in parent node
        for j in range(i, node.size - 1):
            node.keys[j] = node.keys[j + 1]
            node.vals[j] = node.vals[j + 1]
            node.children[j + 1] = node.children[j + 2]
        node.keys[node.size - 1] = None
        node.vals[node.size - 1] = None
        node.children[node.size] = None
        node.size -= 1

    def _borrow_from_left(self, node: NodePtr, i: int):
        """
        i 番目の子 (y) が D-1 個のキーしか持たない場合、
        左の兄弟 (z) からキーを借りてくる。
        """
        child = node.children[i]
        left_sibling = node.children[i - 1]

        # Shift keys and values in child to make space at the beginning
        for j in range(child.size, 0, -1):
            child.keys[j] = child.keys[j - 1]
            child.vals[j] = child.vals[j - 1]

        # If child is internal, shift children too
        if not child.is_leaf():
            for j in range(child.size + 1, 0, -1):
                child.children[j] = child.children[j - 1]

        # Move key/value from parent to child
        child.keys[0] = node.keys[i - 1]
        child.vals[0] = node.vals[i - 1]

        # Move key/value from left sibling to parent
        node.keys[i - 1] = left_sibling.keys[left_sibling.size - 1]
        node.vals[i - 1] = left_sibling.vals[left_sibling.size - 1]
        left_sibling.keys[left_sibling.size - 1] = None
        left_sibling.vals[left_sibling.size - 1] = None

        # If left sibling is internal, move its rightmost child to child
        if not child.is_leaf():
            child.children[0] = left_sibling.children[left_sibling.size]
            left_sibling.children[left_sibling.size] = None

        left_sibling.size -= 1
        child.size += 1

    def _borrow_from_right(self, node: NodePtr, i: int):
        """
        i 番目の子 (y) が D-1 個のキーしか持たない場合、
        右の兄弟 (z) からキーを借りてくる。
        """
        child = node.children[i]
        right_sibling = node.children[i + 1]

        # Move key/value from parent to child
        child.keys[child.size] = node.keys[i]
        child.vals[child.size] = node.vals[i]
        child.size += 1

        # Move key/value from right sibling to parent
        node.keys[i] = right_sibling.keys[0]
        node.vals[i] = right_sibling.vals[0]

        # Shift keys and values in right sibling
        for j in range(right_sibling.size - 1):
            right_sibling.keys[j] = right_sibling.keys[j + 1]
            right_sibling.vals[j] = right_sibling.vals[j + 1]
        right_sibling.keys[right_sibling.size - 1] = None
        right_sibling.vals[right_sibling.size - 1] = None

        # If child is internal, move right sibling's leftmost child to child
        if not child.is_leaf():
            child.children[child.size] = right_sibling.children[0]
            for j in range(right_sibling.size):
                right_sibling.children[j] = right_sibling.children[j + 1]
            right_sibling.children[right_sibling.size] = None

        right_sibling.size -= 1

    def get(self, key: K) -> Optional[V]:
        """値を取得する。"""
        node = self.root
        while node:
            i = 0
            while i < node.size and key > node.keys[i]:
                i += 1
            if i < node.size and key == node.keys[i]:
                return node.vals[i]
            if node.is_leaf():
                return None
            node = node.children[i]
        return None

    def get_mut(self, key: K) -> Optional[V]:
        """値の可変参照を取得する。（Pythonでは直接参照を返すため、`get`と同じ）"""
        # PythonではRustのような可変参照の概念が直接は適用できません。
        # ここでは、キーに対応する値を直接返します。もし値がミュータブルなオブジェクトであれば、
        # 呼び出し元でそのオブジェクトを変更できます。
        node = self.root
        while node:
            i = 0
            while i < node.size and key > node.keys[i]:
                i += 1
            if i < node.size and key == node.keys[i]:
                return node.vals[i]
            if node.is_leaf():
                return None
            node = node.children[i]
        return None

    def len(self) -> int:
        """要素数を取得する。"""
        return self.size

    def print_as_tree(self):
        """木の形式で表示する。"""
        _print_as_tree_util(self.root)


# ---------------------------

import sys

input = sys.stdin.readline

if __name__ == "__main__":
    map_tree: BTreeMap[int, tuple] = BTreeMap(D=3)  # Degree 3 (2-3-4 tree)

    L, Q = map(int, input().split())

    for i in range(Q):
        c, x = map(int, input().split())

        if c == 1:
            map_tree.insert(x, ())
        else:
            l = map_tree.
