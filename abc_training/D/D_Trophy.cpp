//                D - Trophy               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc258/tasks/abc258_d
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
constexpr ll INF = 1010101010101;

int main() {
    int N, X; cin >> N >> X;
    vector<ll> A(N), B(N);
    rep(i, 0, N) cin >> A[i] >> B[i];

    ll sum=0, mini=INF, ans=INF;
    rep(i, 0, min(X, N)) {
        ll tmp = sum + mini * (X-i);
        if (tmp > 0 && ans > tmp) {
            ans = tmp;
        }
        sum += A[i] + B[i];
        chmin(mini, B[i]);
    }

    cout << ans << endl;
}
