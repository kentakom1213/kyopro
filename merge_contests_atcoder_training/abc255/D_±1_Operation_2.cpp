//            D - ±1 Operation 2           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc255/tasks/abc255_d
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
    int N, Q; cin >> N >> Q;
    vector<ll> A(N), S(N+1);
    rep(i, 0, N) cin >> A[i];
    sort(ALL(A));
    rep(i, 0, N) S[i+1] = S[i] + A[i];  // 累積和を取る

    print_vector(A);

    // クエリ処理
    while (Q--) {
        ll q; cin >> q;
        int lb = distance(A.begin(), lower_bound(ALL(A), q));
        ll left = q*lb - S[lb];
        ll right = q*(N-lb) - (S[N] - S[lb]);
        cout << left - right << endl;
    }
}
