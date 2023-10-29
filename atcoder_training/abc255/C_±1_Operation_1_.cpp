//           C - ±1 Operation 1            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc255/tasks/abc255_c
// ----------------------------------------

/*

A + Dk <= X < A + D(k+1)
を満たすkに対して，
min{ |A + Dk - X|,  |A + D(k+1) - X|}
を求める．
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    ll X, A, D, N;
    cin >> X >> A >> D >> N;

    ll sub = min(A, A + D * (N-1));
    ll sup = max(A, A + D * (N-1));

    // 範囲外の場合
    if (X <= sub || sup <= X) {
        cout << abs(min(sup-X, X-sub)) << endl;
        return 0;
    }

    // A + Dk <= X < A + D(k+1)　を満たすkを求める
    ll k = (X - A) / D;
    ll ans = min( abs(A + D*k - X), abs(A + D*(k+1) - X) );
    cout << ans << endl;
}
