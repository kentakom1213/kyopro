//            D - Sequence Query           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc241/tasks/abc241_d

// これ自力では厳しそう
// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int Q; cin >> Q;
    multiset<ll> mset;
    while (Q--) {
        ll q, x; cin >> q >> x;
        if (q == 1) {
            mset.insert(x);
        }
        else {
            int k; cin >> k;
            bool ng = false;
            if (q == 2) {
                auto itr = mset.upper_bound(x);
                while (k--) {
                    if (itr==mset.begin()) {
                        ng = true;
                        break;
                    }
                    itr--;
                }
                if (ng) cout << -1 << endl;
                else cout << (*itr) << endl;
            }
            if (q == 3) {
                auto itr = mset.lower_bound(x);
                while (k--) {
                    if (itr==mset.end()) {
                        ng = true;
                        break;
                    }
                    if (k==0) break;
                    itr++;
                }
                if (ng) cout << -1 << endl;
                else cout << (*itr) << endl;
            }
        }
    }
}
