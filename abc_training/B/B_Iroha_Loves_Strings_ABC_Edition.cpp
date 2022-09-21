//  B - Iroha Loves Strings (ABC Edition)  
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc042/tasks/abc042_b
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N, L; cin >> N >> L;
    vector<string> S(N);
    for (int i=0; i<N; i++) cin >> S[i];
    sort(ALL(S));
    for (auto s : S) cout << s;
    cout << endl;
}
