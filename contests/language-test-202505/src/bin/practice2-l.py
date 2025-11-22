from typing import Callable, Optional


def bit_length(n: int) -> int:
    if n < 0:
        n = -n

    length = 0
    while n > 0:
        n >>= 1  # 右に1ビットシフト
        length += 1
    return length


class LazySegmentTree[T, F]:
    """遅延セグメント木"""

    # 定数
    _size: int
    _offset: int

    # 関数
    _id_t: Callable[[], T]
    _id_f: Callable[[], F]
    _op: Callable[[T, T], T]
    _mapping: Callable[[T, F], T]
    _composition: Callable[[F, F], F]
    _aggregation: Callable[[F, int], F]

    # データ
    _data: list[T]
    _lazy: list[F]

    def __init__(
        self,
        n: int,
        id_t: Callable[[], T],
        id_f: Callable[[], F],
        op: Callable[[T, T], T],
        mapping: Callable[[T, F], T],
        composition: Callable[[F, F], F],
        aggregation: Callable[[F, int], F],
    ) -> None:
        """遅延セグメント木を初期化する

        Args:
            n (int): 木のサイズ
            id_t (Callable[[], T]): 値の単位元
            id_f (Callable[[], F]): 作用の単位元
            op (Callable[[T, T], T]): 二項演算
            mapping (Callable[[T, F], T]): 作用の適用
            composition (Callable[[F, F], F]): 作用の合成
            aggregation (Callable[[F, int], F]): 作用の繰り返し
        """
        # 定数
        self._size = n
        self._offset = 1 << bit_length(n - 1)

        # 関数
        self._id_t = id_t
        self._id_f = id_f
        self._op = op
        self._mapping = mapping
        self._composition = composition
        self._aggregation = aggregation

        # データ
        self._data = [id_t() for _ in range(self._offset * 2)]
        self._lazy = [id_f() for _ in range(self._offset * 2)]

    def __eval(self, i: int, len: int):
        """遅延評価を行う

        Args:
            i (int): 遅延評価を行うノードのインデックス
            len (int): 遅延評価を行うノードの長さ
        """
        if self._lazy[i] == self._id_f():
            return

        # 葉でなければ子に伝搬
        if i < self._offset:
            self._lazy[i * 2] = self._composition(self._lazy[i * 2], self._lazy[i])
            self._lazy[i * 2 + 1] = self._composition(
                self._lazy[i * 2 + 1], self._lazy[i]
            )

        # 値の更新
        self._data[i] = self._mapping(
            self._data[i], self._aggregation(self._lazy[i], len)
        )
        self._lazy[i] = self._id_f()

    def apply(
        self,
        left: int,
        right: int,
        val: F,
        begin: int = 0,
        end: Optional[int] = None,
        idx: int = 1,
    ):
        """区間 [l, r) に作用を適用する

        Args:
            left (int): 区間の左端
            right (int): 区間の右端
            val (F): 作用
        """
        if end is None:
            end = self._offset

        # 遅延値を評価
        self.__eval(idx, end - begin)

        # 区間を内包するとき
        if left <= begin and end <= right:
            self._lazy[idx] = self._composition(self._lazy[idx], val)
            self.__eval(idx, end - begin)

        # 区間が重なるとき
        elif left < end and begin < right:
            mid = (begin + end) // 2
            # 左の子を更新
            self.apply(left, right, val, begin, mid, idx * 2)
            # 右の子を更新
            self.apply(left, right, val, mid, end, idx * 2 + 1)
            # 値を更新
            self._data[idx] = self._op(self._data[idx * 2], self._data[idx * 2 + 1])

    def get_range(
        self,
        left: int,
        right: int,
        begin: int = 0,
        end: Optional[int] = None,
        idx: int = 1,
    ):
        """区間 [l, r) の値を取得する

        Args:
            left (int): 区間の左端
            right (int): 区間の右端
        """
        if end is None:
            end = self._offset

        # 遅延値を評価
        self.__eval(idx, end - begin)

        # 区間が交差しないとき
        if end <= left or right <= begin:
            return self._id_t()

        # 区間を内包するとき
        if left <= begin and end <= right:
            return self._data[idx]

        # 区間が重なるとき
        mid = (begin + end) // 2
        left_val = self.get_range(left, right, begin, mid, idx * 2)
        right_val = self.get_range(left, right, mid, end, idx * 2 + 1)
        return self._op(left_val, right_val)


def from_array[T, F](
    array: list[T],
    id_t: Callable[[], T],
    id_f: Callable[[], F],
    op: Callable[[T, T], T],
    mapping: Callable[[T, F], T],
    composition: Callable[[F, F], F],
    aggregation: Callable[[F, int], F],
) -> LazySegmentTree[T, F]:
    """配列から遅延セグメント木を構築する

    Args:
        array (list[T]): 配列
        id_t (Callable[[], T]): 値の単位元
        id_f (Callable[[], F]): 作用の単位元
        op (Callable[[T, T], T]): 二項演算
        mapping (Callable[[T, F], T]): 作用の適用
        composition (Callable[[F, F], F]): 作用の合成
        aggregation (Callable[[F, int], F]): 作用の繰り返し

    Returns:
        LazySegmentTree[T, F]: 構築したセグメント木
    """
    seg = LazySegmentTree(len(array), id_t, id_f, op, mapping, composition, aggregation)
    seg._data[seg._offset : seg._offset + len(array)] = array

    for i in range(seg._offset - 1, 0, -1):
        seg._data[i] = seg._op(seg._data[i * 2], seg._data[i * 2 + 1])

    return seg


XOR_INV = (
    # id_t
    lambda: (0, 0, 0),
    # id_f
    lambda: False,
    # op
    lambda x, y: (x[0] + y[0], x[1] + y[1], x[2] + y[2] + x[1] * y[0]),
    # mapping
    lambda x, f: x if not f else (x[1], x[0], x[1] * x[0] - x[2]),
    # composition
    lambda f, g: f ^ g,
    # aggregation
    lambda f, n: f,
)


def main():
    N, Q = list(map(int, input().split()))
    A = list(map(int, input().split()))

    seg = from_array(
        [(1 - a, a, 0) for a in A],
        lambda: (0, 0, 0),
        lambda: False,
        lambda x, y: (x[0] + y[0], x[1] + y[1], x[2] + y[2] + x[1] * y[0]),
        lambda x, f: x if not f else (x[1], x[0], x[1] * x[0] - x[2]),
        lambda f, g: f ^ g,
        lambda f, n: f,
    )

    for _ in range(Q):
        t, l, r = list(map(int, input().split()))

        if t == 1:
            # 更新クエリ
            seg.apply(l - 1, r, True)
        else:
            # 取得クエリ
            res = seg.get_range(l - 1, r)

            print(res[2])


main()
