//            D - Integer Cards            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc127/tasks/abc127_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

using pll = pair<ll, ll>;
ll N, M;

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
    cin >> N >> M;
    vector<ll> A(N);
    vector<pll> CB(M);
    rep(i, 0, N) cin >> A[i];
    rep(i, 0, M) cin >> CB[i].second >> CB[i].first;

    sort(ALL(A));
    sort(ALL(CB), greater<pll>());


    ll i=0, ans=0;
    for (auto [c, b] : CB) {
        rep(_, 0, b) {
            if (i == N) break;
            ans += max(c, A[i]);
            i++;
        }
        if (i == N) break;
    }

    // 残りを足す
    for (int j=i; j<N; j++) {
        ans += A[j];
    }

    cout << ans << endl;
}
