//                C - Coupon               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc246/tasks/abc246_c
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

ll N, K, X;

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
    cin >> N >> K >> X;
    vector<ll> A(N);
    rep(i, 0, N) {
        cin >> A[i];
        ll minus = min(K, A[i]/X);
        A[i] -= X * minus;
        K -= minus;
    }

    // ソート
    sort(ALL(A), greater<ll>());

    ll ans = 0;
    rep(i, 0, N) {
        if (K) K--;
        else ans += A[i];
    }

    cout << ans << endl;
}
