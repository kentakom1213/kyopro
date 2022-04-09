//              B - Robot Arms             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/keyence2020/tasks/keyence2020_b
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int INF = 1<<28;
typedef pair<int, int> P;

int main() {
    int N; cin >> N;
    vector<P> arms(N);

    rep(i, N) {
        int x, l; cin >> x >> l;
        arms[i] = {x+l, x-l};  // {右腕, 左腕}
    }

    // 終端でソート
    sort(ALL(arms));

    // 貪欲にとる
    int ans=0, now_r=-INF;
    for (auto [r, l] : arms) {
        if (now_r <= l) {
            ans++;
            now_r = r;
        }
    }

    cout << ans << endl;
}
