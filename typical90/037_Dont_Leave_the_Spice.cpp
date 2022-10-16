//     037 - Don't Leave the Spice（★5）     
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ak
// ----------------------------------------

/*

## 愚直な解法
dpをする
dp[i][j] := i個めまでの料理で合計j[mg]の香辛料を使っているときの価値の最大値

## 方針
- セグ木でdpを高速化

*/

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

// FUNCTIONS
using X = ll;

X fx(X a, X b) {
    return max(a, b);
}

X e() {
    return -1;
}

template <class X, X (*op)(X, X), X (*e)()>
struct SegTree {
    using FX = function<X(X, X)>;  // X*X -> X となる関数
    int offset;
    int tree_size;
    // FX fx;  // 写像
    // const X ex;  // 単位元
    vector<X> tree;

    SegTree(int n) : offset(), tree_size(), tree() {
        // offset, tree_size の初期化
        _set_size(n);
        // tree の初期化
        tree.assign(tree_size, e());
    }

    SegTree(vector<X> vec) : offset(), tree_size(), tree() {
        // arrayの初期化
        _set_size(vec.size());
        tree.assign(tree_size, e());

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
        X res = e();

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

    void show_arr() {
        /* 配列を表示 */
        cout << "{ ";
        for (int i = offset; i < offset + tree_size/2; i++) {
            cout << tree[i] << " "; 
        }
        cout << "}" << "\n";
    }
};

int main() {
    ll w, n; cin >> w >> n;

    vector<SegTree<X, fx, e>> dp(n+1, SegTree<X, fx, e>(w+1));
    dp[0].update(0, 0);
    // dp[0].show_arr(); cout << "\n";

    for (int i = 1; i <= n; i++) {
        ll l, r, v; cin >> l >> r >> v;
        for (ll j = 0; j <= w; j++) {
            // 下に下ろす
            if (dp[i].get_point(j) < dp[i-1].get_point(j)) {
                dp[i].update(j, dp[i-1].get_point(j));
            }

            // l <= k <= r について、dp[i-1][j-k] の最大値を取得
            ll maxi = dp[i-1].get_range(
                max(0LL, j-r),
                min(w, j-l+1)
            );
            if (maxi >= 0 && dp[i].get_point(j) < maxi + v) {
                dp[i].update(j, maxi + v);
            }
        }

        // dp[i].show_arr();
        // cout << endl;
    }

    ll ans = dp[n].get_point(w);
    cout << ans << endl;
}
