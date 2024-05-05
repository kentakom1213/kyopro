//         D - Left Right Operation        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc263/tasks/abc263_d
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

int main() {
    ll N, L, R; cin >> N >> L >> R;
    vector<ll> A(N);
    rep(i, 0, N) cin >> A[i];

    // 累積和
    vector<ll> S(N+1);
    rep(i, 0, N) S[i+1] = S[i] + A[i];

    // Lの埋め方
    ll left = 0;  // i < left の要素をLで埋める
    ll fill_min = (ll)1e20;
    rep(i, 0, N+1) {
        ll i_to_N = S[N] - S[i];
        if (fill_min > L * i + i_to_N) {
            fill_min = L * i + i_to_N;
            left = i;
        }
    }

    // 累積和を更新
    rep(i, 0, left) A[i] = L;
    rep(i, 0, N) S[i+1] = S[i] + A[i];
    
    // Rの埋め方
    fill_min = (ll)1e20;
    rep(i, 0, N) {
        chmin(fill_min, S[i+1] + R * (N-i-1));
    }

    cout << fill_min << endl;
}
