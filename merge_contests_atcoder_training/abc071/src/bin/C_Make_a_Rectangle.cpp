//           C - Make a Rectangle          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc071/tasks/arc081_a
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
    int N; cin >> N;
    map<ll, ll> cnt;
    rep(i, 0, N) {
        ll a; cin >> a;
        cnt[a]++;
    }
    ll ans=0, tmp=0;
    for (auto itr=cnt.rbegin(); itr!=cnt.rend(); itr++) {
        auto [k, v] = *itr;
        if (tmp && v >= 2) {
            ans = tmp * k;
            break;
        } else if (v >= 4) {
            ans = k * k;
            break;
        } else if (v >= 2) {
            tmp = k;
        }
    }
    cout << ans << endl;
}
