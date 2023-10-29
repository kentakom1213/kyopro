//          D - Union of Interval          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc256/tasks/abc256_d
// ----------------------------------------

/* 区間の更新
 * imosが便利
 */

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
constexpr ll INF = numeric_limits<long long>::max();

int main() {
    int N; cin >> N;
    vector<int> imos(202020);

    rep(i, 0, N) {
        int l, r; cin >> l >> r;
        imos[l]++;
        imos[r]--;
    }

    // 累積和
    ll ans = 0;
    rep(i, 0, 202010) {
        imos[i+1] = imos[i] + imos[i+1];
    }

    // 1以上の要素になる場合に出力
    ll prev = 0;
    rep(i, 0, 202020) {
        if (prev > 0 && imos[i] == 0) {
            cout << i << endl;
        } else if (prev == 0 && imos[i] > 0) {
            cout << i << " ";
        }
        prev = imos[i];
    }
}
