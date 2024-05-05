from sys import stdin
from typing import Generic, Tuple, TypeVar, Iterator, Optional

K = TypeVar("K")
V = TypeVar("V")


class AANode(Generic[K, V]):
    """AA木のノード
    """

    def __init__(self, key, val, level=1, left=None, right=None) -> None:
        self.key = key
        self.val = val
        self.level = level
        self.left = left
        self.right = right

    def __repr__(self) -> str:
        return f"AANode(key={repr(self.key)}, val={repr(self.val)}, level={self.level}, left={self.left}, right={self.right})"


def _skew(root: AANode) -> AANode:
    """skew操作を行う

    ```text
      |        ⇓           ⇓        
    2 |    L ← T           L → T    
      |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
    1 |  A   B   R       A   B   R  
    ```
    """
    if root is None:
        return None
    if root.left is None:
        return root
    elif root.level == root.left.level:
        # ポインタの入れ替え
        L: AANode = root.left
        root.left = L.right
        L.right = root
        return L
    else:
        return root


def _split(root: AANode) -> AANode:
    """split操作を行う

    ```text
      |                         ⇓    
    3 |                         R    
      |    ⇓                   ↙ ↘   
    2 |    T → R → Δ   ==>    T   Δ  
      |   ↙   ↙              ↙ ↘     
    1 |  A   B              A   B    
    ```
    """
    if root is None:
        return None
    if root.right is None or root.right.right is None:
        return root
    elif root.level == root.right.right.level:
        # ポインタの入れ替え
        R: AANode = root.right
        root.right = R.left
        R.left = root
        R.level += 1
        return R
    else:
        return root


def insert(root: AANode, key, val) -> Tuple[AANode, bool]:
    """keyに対応するvalを挿入する．

    Args:
        root (AANode): 挿入対象の木
        key (K): 挿入するキー
        val (V): 挿入する値

    Returns:
        Tuple[AANode, bool]: (挿入後の木, 挿入が行われたかどうか)
    """
    if root is None:
        return AANode(key, val), True
    if key < root.key:
        root.left, is_inserted = insert(root.left, key, val)
    elif key > root.key:
        root.right, is_inserted = insert(root.right, key, val)
    else:
        root.val = val
        is_inserted = False
    root = _skew(root)
    root = _split(root)
    return root, is_inserted


def _take_max(root: AANode) -> Tuple[AANode, Optional[Tuple[K, V]]]:
    """部分木の最大値を持ってくる
    """
    if root is None:
        return (None, None)

    new_right, right_most = _take_max(root.right)
    if right_most is None:
        return (None, (root.key, root.val))
    root.right = new_right
    # 再平衡化
    root = _rebarance(root)
    return (root, right_most)


def _rebarance(root: AANode) -> AANode:
    """再帰的に再平衡化を行う
    """
    if root is None:
        return None
    # 左，右の高さ
    llevel = root.left.level if root.left is not None else 0
    rlevel = root.right.level if root.right is not None else 0
    if llevel < root.level - 1 or rlevel < root.level - 1:
        root.level -= 1
        # 右が低い場合，下げる
        if rlevel > root.level:
            root.right.level = root.level
        # 自分，右の子，右の右の子までskew
        root = _skew(root)
        root.right = _skew(root.right)
        root.right.right = _skew(root.right.right)
        # 自分，右の子までsplit
        root = _split(root)
        root.right = _split(root.right)
    return root


def delete(root: AANode, key) -> Tuple[AANode, bool]:
    """keyに一致するノードが存在する場合，それを削除する．

    Args:
        root (AANode): 削除対象の木
        key (K): 削除するキー

    Returns:
        Tuple[AANode, bool]: (削除後の木, 削除が行われたかどうか)
    """
    if root is None:
        return None, False
    if key < root.key:
        root.left, is_deleted = delete(root.left, key)
    elif key > root.key:
        root.right, is_deleted = delete(root.right, key)
    else:
        if root.left is None and root.right is None:
            root = None
        elif root.left is None:
            root = root.right
        elif root.right is None:
            root = root.left
        else:
            # 左部分木の最大値を持ってくる
            new_left, (k, v) = _take_max(root.left)
            root.left = new_left
            root.key = k
            root.val = v
        is_deleted = True
    root = _rebarance(root)
    return root, is_deleted


def get(root: AANode, key):
    """keyに一致する値が存在する場合，それを返す．
    存在しない場合にはNoneを返す．

    Args:
        root (AANode): 検索対象の木
        key=検索するキー
    """
    while root is not None:
        if key < root.key:
            root = root.left
        elif key > root.key:
            root = root.right
        else:
            return root.val
    return None


def _binary_search(root: AANode, key, cmp):
    """二分探索により，
    - `cmp(key, lo) -> False`
    - `cmp(key, hi) -> True`

    を満たす`lo`,`hi`の組を探索する
    """
    root = root
    lo = None
    hi = None
    while root is not None:
        if cmp(key, root.key):
            hi = root
            root = root.left
        else:
            lo = root
            root = root.right
    return (
        (lo.key, lo.val) if lo is not None else None,
        (hi.key, hi.val) if hi is not None else None
    )


def iter_node(root: AANode) -> Iterator[Tuple[K, V]]:
    """木を巡回する
    """
    if root is not None:
        yield from iter_node(root.left)
        yield (root.key, root.val)
        yield from iter_node(root.right)


class AATreeSet(Generic[K, V]):
    """AA木による順序付き集合の実装
    """

    def __init__(self, root=None, size=0) -> None:
        self._root = root
        self._size = size

    def add(self, key, val):
        """keyに対応するvalを挿入する．

        Args:
            key (K): 挿入するキー
            val (V): 挿入する値
        """
        self._root, is_inserted = insert(self._root, key, val)
        self._size += is_inserted

    def discard(self, key):
        """keyに一致するノードが存在する場合，それを削除する．

        Args:
            key (K): 削除するキー
        """
        self._root, is_deleted = delete(self._root, key)
        self._size -= is_deleted

    def __getitem__(self, key) -> V:
        """keyに一致する値が存在する場合，それを返す．

        Args:
            key (K): 検索するキー
        """
        val = get(self._root, key)
        if val is None:
            raise KeyError

    def le(self, key):
        """key以下の最大の値を返す
        存在しない場合にはNoneを返す．
        """
        return _binary_search(self._root, key, lambda x, y: x < y)[0]

    def lt(self, key):
        """key以下の最大の値を返す
        存在しない場合にはNoneを返す．
        """
        return _binary_search(self._root, key, lambda x, y: x <= y)[0]

    def ge(self, key):
        """key以上の最小の値を返す．
        存在しない場合にはNoneを返す．
        """
        return _binary_search(self._root, key, lambda x, y: x <= y)[1]

    def gt(self, key):
        """keyより大きい最小の値を返す．
        存在しない場合にはNoneを返す．
        """
        return _binary_search(self._root, key, lambda x, y: x < y)[1]

    def __len__(self):
        return self._size

    def __repr__(self) -> str:
        return f"AATreeSet(root={self._root}, size={self._size})"

    def __str__(self) -> str:
        arr = list(self)
        return f"AATreeSet({arr})"

    def __iter__(self):
        yield from iter_node(self._root)

    def __contains__(self, key):
        return get(self._root, key) is not None

# ------------- main -------------


def main():
    # 入力高速化
    readline = stdin.readline

    N, Q = map(int, readline().split())

    called = AATreeSet()
    last = 0

    for _ in range(Q):
        t, *x = map(int, readline().split())

        if t == 1:
            last += 1
            called.add(last, None)
        elif t == 2:
            called.discard(x[0])
        else:
            fst, _ = called.gt(-1)
            print(fst)


main()
