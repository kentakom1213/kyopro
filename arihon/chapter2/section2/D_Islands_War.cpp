//             D - Islands War             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc103/tasks/abc103_d
// ----------------------------------------

/*
区間スケジューリングって言われないとわかんないかも
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
typedef pair<int, int> P;

int main() {
    int N, M; cin >> N >> M;
    vector<P> wars(M);
    rep(i, M) {
        int l, r; cin >> l >> r;
        wars[i] = {r-1, l-1};
    }

    // 終端についてソート
    sort(ALL(wars));

    // 貪欲にとる
    int ans=0, now=0;
    for (auto [r, l] : wars) {
        if (now <= l) {
            ans++;
            now = r;
        }
    }

    cout << ans << endl;
}
