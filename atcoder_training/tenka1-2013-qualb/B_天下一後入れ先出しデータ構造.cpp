//            B - 天下一後入れ先出しデータ構造           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/tenka1-2013-qualb/tasks/tenka1_2013_qualB_b
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
using pll = pair<ll, ll>;

int main() {
    ll q, l;
    cin >> q >> l;

    ll size = 0;
    stack<pll> st;

    string com;
    while (q--) {
        cin >> com;
        if (com == "Push") {
            ll n, m; cin >> n >> m;
            size += n;
            if (size > l) {
                cout << "FULL" << endl;
                return 0;
            }
            st.push({m, n});
        }
        else if (com == "Pop") {
            ll n; cin >> n;
            size -= n;
            if (size < 0) {
                cout << "EMPTY" << endl;
                return 0;
            }
            while (n && !st.empty()) {
                auto [x, y] = st.top();
                st.pop();
                if (n >= y) {
                    n -= y;
                }
                else {
                    st.push({x, y-n});
                    break;
                }
            }
        }
        else if (com == "Top") {
            if (size == 0) {
                cout << "EMPTY" << endl;
                return 0;
            }
            auto [x, y] = st.top();
            cout << x << endl;
        }
        else {
            cout << size << endl;
        }
    }
    cout << "SAFE" << endl;
}
