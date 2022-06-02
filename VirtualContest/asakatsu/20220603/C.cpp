// https://atcoder.jp/contests/abc117/tasks/abc117_c

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
    int N, M; cin >> N >> M;
    if (N >= M) {
        cout << 0 << endl;
        return 0;
    }

    vector<int> X(M), diff(M-1);
    rep(i, 0, M) cin >> X[i];
    sort(ALL(X));
    rep(i, 0, M-1) diff[i] = X[i+1] - X[i];  // 累積和
    sort(ALL(diff));
    ll ans = 0;
    rep(i, 0, M-N) ans += diff[i];
    cout << ans << endl;
}

