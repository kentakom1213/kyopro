#include <iostream>
#include <vector>
using namespace std;

struct SegTreeMin {
    long long inf = (1LL << 31) - 1;
    int offset;
    int arr_size;
    vector<long long> arr;

    SegTreeMin(int N) {
        // Nより大きい2の冪数
        int n2 = 0;
        N--;
        while (N) {
            n2++;
            N >>= 1;
        }

        // arrayの初期化
        offset = 1 << n2;
        arr_size = offset << 1;
        arr.assign(arr_size, inf);
    }

    void update(int i, long long x) {
        /* arr[i] を x で置き換える */
        i += offset;  // 1-indexed に変換
        arr[i] = x;

        // treeを遡る
        while (i > 1) {
            i >>= 1;
            int prev = i << 1;
            arr[i] = min(arr[prev], arr[prev + 1]);
        }
    }

    long long get_min(int l, int r) {
        /* [l,r) の範囲を検索 */
        long long res = inf;

        l += offset;
        r += offset;
        while (l < r) {
            if (r & 1) {
                res = min(res, arr[r-1]);
            }
            if (l & 1) {
                res = min(res, arr[l]);
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
        while (i < arr_size) {
            cout << arr[i] << " ";
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
int main() {
    int N, Q;
    cin >> N >> Q;

    SegTreeMin seg(N);

    while (Q--) {
        int com, x, y;
        cin >> com >> x >> y;
        if (com == 0) {
            seg.update(x, y);
        }
        else if (com == 1) {
            long long v = seg.get_min(x, y+1);
            cout << v << endl;
        }
    }
}