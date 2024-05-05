//         D - Left Right Operation        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc263/tasks/abc263_d

// 公式解説準拠
// https://atcoder.jp/contests/abc263/editorial/4549
// ----------------------------------------

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

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int main() {
    ll N, L, R; cin >> N >> L >> R;
    vector<ll> A(N);
    rep(i, 0, N) cin >> A[i];

    // f,gをもとめる
    vector<ll> f(N+1), g(N+1);
    rep(i, 0, N) {
        f[i+1] = min(f[i]+A[i], L*(i+1));
        g[i+1] = min(g[i]+A[N-i-1], R*(i+1));
    }

    // 最小値を求める
    ll ans = (ll)1e20;
    rep(i, 0, N+1) {
        chmin(ans, f[i] + g[N-i]);
    }

    cout << ans << endl;
}
