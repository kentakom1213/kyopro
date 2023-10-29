//           D - Cutting Woods
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc217/tasks/abc217_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    ll L, Q; cin >> L >> Q;
    set<ll> wood;
    wood.insert(0);
    wood.insert(L);

    while (Q--) {
        ll c, x; cin >> c >> x;
        if (c == 1) {
            wood.insert(x);
        } else {
            auto pright = wood.lower_bound(x);
            ll right=*pright, left=*(--pright);
            cout << right - left << endl;
        }
    }
}
