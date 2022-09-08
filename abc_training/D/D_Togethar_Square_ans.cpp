//           D - Together Square           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc254/tasks/abc254_d

// 解説
// https://atcoder.jp/contests/abc254/editorial/4065
// ----------------------------------------

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
    ll N; cin >> N;

    // 平方数判定
    vector<bool> isSq(N+1, false);
    for (int i=1; i*i <= N; i++) isSq[i*i] = true;

    // 約数列挙
    vector<vector<int>> d(N+1);
    for (int i=1; i <= N; i++) {
        for (int j = i; j <= N; j += i) {
            d[j].emplace_back(i);
        }
    }

    // i/f(i) を求める
    vector<int> cnt(N+1);
    for (int i = 1; i <= N; i++) {
        // 約数の中で最大の平方数 f(i) を求める
        int f = 0;
        for (int v : d[i]) if (isSq[v]) f = v;
        cnt[i/f]++;
    }

    // 数え上げ
    ll ans = 0;
    for (int i = 1; i <= N; i++) ans += cnt[i] * cnt[i];
    cout << ans << endl;
}
