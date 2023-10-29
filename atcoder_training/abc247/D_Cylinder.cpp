//               D - Cylinder              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc247/tasks/abc247_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    deque<pll> deq;

    int q; cin >> q;
    while (q--) {
        int type; cin >> type;
        if (type == 1) {
            int x, c; cin >> x >> c;
            if (deq.empty() || deq.rbegin()->first != x) {
                deq.push_back( {x, c} );
            }
            else {
                deq.rbegin()->second += c;
            }
        }
        else {
            int c; cin >> c;
            ll ans = 0;
            while (c) {
                auto &[front_x, front_c] = deq.front();
                if (c >= front_c) {
                    c -= front_c;
                    ans += front_x * front_c;
                    deq.pop_front();
                }
                else {
                    front_c -= c;
                    ans += front_x * c;
                    break;
                }
            }
            cout << ans << endl;
        }
    }
}
