#include <iostream>
#include <vector>
#include <functional>
using namespace std;

template<class X>
struct SegTree {
    using FX = function<X(X, X)>;  // X*X -> X となる関数
    int offset;
    int tree_size;
    FX fx;  // 写像
    const X ex;  // 単位元
    vector<X> tree;

    SegTree(int n, FX fx_, X ex_) : offset(), tree_size(), fx(fx_), ex(ex_), tree() {
        // offset, tree_size の初期化
        _set_size(n);
        // tree の初期化
        tree.assign(tree_size, ex);
    }

    template<class T>
    SegTree(vector<T> vec, FX fx_, X ex_) : offset(), tree_size(), fx(fx_), ex(ex_), tree() {
        // arrayの初期化
        _set_size(vec.size());
        tree.assign(tree_size, ex);

        // vecを代入
        for (int i = 0; i < vec.size(); i++) {
            tree[i + offset] = vec[i];
        }

        // 親要素を初期化
        for (int i = offset-1; i > 0; i--) {
            int prev = i << 1;
            tree[i] = fx(tree[prev], tree[prev + 1]);
        }
    }

    void _set_size(int n) {
        /* tree_size, offset を初期化する */
        // nより大きい2の冪数
        int n2 = 0;
        n--;
        while (n) {
            n2++;
            n >>= 1;
        }
        offset = 1 << n2;
        tree_size = offset << 1;
    }

    void update(int i, X x) {
        /* tree[i] を x で置き換える */
        i += offset;  // 1-indexed に変換
        tree[i] = x;

        // treeを遡る
        while (i > 1) {
            i >>= 1;
            int prev = i << 1;
            tree[i] = fx(tree[prev], tree[prev + 1]);
        }
    }

    X get_point(int i) {
        /* 一点を取得 */
        return tree[offset + i];
    }

    X get_range(int l, int r) {
        /* [l,r) の範囲を検索 */
        X res = ex;

        l += offset;
        r += offset;
        while (l < r) {
            if (r & 1) {
                res = fx(res, tree[r-1]);
            }
            if (l & 1) {
                res = fx(res, tree[l]);
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }

        return res;
    }

    void show() {
        /* Tree状に表示 */
        int i=1, col=2;
        while (i < tree_size) {
            cout << tree[i] << " ";
            i++;
            if (i == col) {
                cout << "\n";
                col <<= 1;
            }
        }
    }
};

// test
// [Range Minimum Query (RMQ)](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A)
// int main() {
//     int N, Q;
//     cin >> N >> Q;

//     SegTree<long long> seg(
//         N,  // データ数
//         [=](long long a, long long b){  // 写像
//             return (a > b ? b : a);
//         },
//         (1LL << 31) - 1  // 単位元
//     );

//     while (Q--) {
//         int com, x, y;
//         cin >> com >> x >> y;
//         if (com == 0) {
//             seg.update(x, y);
//         }
//         else if (com == 1) {
//             long long v = seg.get_range(x, y+1);
//             cout << v << endl;
//         }
//     }
// }

int main() {
    vector<int> A = {1, 2, 3, 4, 5, 6, 7, 8};
    SegTree<long long> seg(
        A,
        [](long long a, long long b){
            return a + b;
        },
        0
    );

    // テスト
    seg.show();
}
