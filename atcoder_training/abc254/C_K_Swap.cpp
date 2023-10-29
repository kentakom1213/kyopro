//                C - K Swap               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc254/tasks/abc254_c
// ----------------------------------------

// modで同値類に分ける
// ↓
// それぞれソート
// ↓
// うまくマージできるか調べる

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
    int N, K; cin >> N >> K;
    vector<vector<int>> REM(K);
    rep(i, 0, N) {
        int a; cin >> a;
        REM[i%K].emplace_back(a);
    }

    // それぞれソート
    for (auto &arr : REM) {
        sort(ALL(arr));
    }

    // 昇順になっているかを確認
    bool isOK = true;
    int now = 0;
    rep(i, 0, N) {
        int val = REM[i%K][i/K];
        isOK &= now <= val;
        now = val;
    }
    cout << (isOK ? "Yes" : "No") << endl;
}
