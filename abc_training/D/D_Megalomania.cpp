//             D - Megalomania             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc131/tasks/abc131_d
// ----------------------------------------

/*
## 方針
- 区間スケジューリング
*/

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

typedef pair<ll, ll> P;

int main() {
    int N; cin >> N;
    vector<P> work(N);
    rep(i, 0, N) {
        ll t, b; cin >> t >> b;
        work[i] = {b, t};
    }
    sort(ALL(work));

    // 区間スケジューリング
    ll now = 0;
    for (auto [b, t] : work) {
        now += t;
        if (now > b) {
            cout << "No" << endl;
            return 0;
        }
    }
    cout << "Yes" << endl;
}
