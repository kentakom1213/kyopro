#include <iostream>
#include <vector>
using namespace std;

struct SegTree {
    long long inf = (1LL << 31) - 1;
    int offset;
    int tree_size;
    vector<long long> tree;

    SegTree(int N) {
        // Nより大きい2の冪数
        int n2 = 0;
        N--;
        while (N) {
            n2++;
            N >>= 1;
        }

        // arrayの初期化
        offset = 1 << n2;
        tree_size = offset << 1;
        tree.assign(tree_size, inf);
    }

    SegTree(vector<long long> vec) {
        int N = vec.size() - 1;
        int n2 = 0;
        while (N) {
            n2++;
            N >>= 1;
        }

        // arrayの初期化
        offset = 1 << n2;
        tree_size = offset << 1;
        tree.assign(tree_size, inf);

        // vecを代入
        for (int i = 0; i < vec.size(); i++) {
            tree[i + offset] = vec[i];
        }

        // 親要素を初期化
        for (int i = offset-1; i > 0; i--) {
            int prev = i << 1;
            tree[i] = min(tree[prev], tree[prev + 1]);
        }
    }

    void update(int i, long long x) {
        /* tree[i] を x で置き換える */
        i += offset;  // 1-indexed に変換
        tree[i] = x;

        // treeを遡る
        while (i > 1) {
            i >>= 1;
            int prev = i << 1;
            tree[i] = min(tree[prev], tree[prev + 1]);
        }
    }

    long long get_min(int l, int r) {
        /* [l,r) の範囲を検索 */
        long long res = inf;

        l += offset;
        r += offset;
        while (l < r) {
            if (r & 1) {
                res = min(res, tree[r-1]);
            }
            if (l & 1) {
                res = min(res, tree[l]);
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

// テスト
int main() {
    int N; cin >> N;
    vector<long long> A(N);
    for(auto &v : A) cin >> v;
    SegTree seg(A);
    seg.show();
}
