//     E - Σ[k=0..10^100]floor(X／10^k)     
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc233/tasks/abc233_e
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int> > name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    string X; cin >> X;
    int d = X.size();

    // 累積和をとる
    vector<ll> S(d+1);
    rep(i, d) {
        S[i+1] = S[i] + (X[i]-'0');
    }

    // 下の桁から順に処理
    stack<int> ans;
    for (int i=d-1; i>=0; i--) {
        ans.push(S[i+1] % 10);
        S[i] += S[i+1] / 10;
    }

    if (S[0]) cout << S[0];
    while (!ans.empty()) {
        cout << ans.top();
        ans.pop();
    }
    cout << endl;
}
