//           C - Robot Takahashi           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc257/tasks/abc257_c
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
    int N; cin >> N;
    string S; cin >> S;
    vector<int> W(N);
    rep(i, 0, N) cin >> W[i];

    // 大人子供を分割
    vector<int> young, adult;
    rep(i, 0, N) {
        if (S[i] == '0') young.push_back(W[i]);
        else adult.push_back(W[i]);
    }
    sort(ALL(young));
    sort(ALL(adult));

    ll ans = max(young.size(), adult.size());
    for (int th : W) {
        // thresholdを設定（th未満は子供，th以上は大人）
        ll pred_young = upper_bound(ALL(young), th) - young.begin();
        ll pred_adult = adult.end() - upper_bound(ALL(adult), th);
        chmax(ans, pred_young + pred_adult);
    }
    cout << ans << endl;
}
