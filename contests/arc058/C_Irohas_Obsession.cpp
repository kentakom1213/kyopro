//          C - Iroha's Obsession          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc058/tasks/arc058_a
// ----------------------------------------

/*
## 方針
- N < 10000だから、6桁の数であれば必ず条件を満たす
- 全探索できる
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

int N, K;

int main() {
    cin >> N >> K;
    vector<int> D(10, 1), C;  // D:使えれば1, C:使える数
    rep(i, K) {
        int d; cin >> d;
        D[d] = 0;
    }
    rep(i, 10) {
        if (D[i]) C.push_back(i);
    }

    // 全探索
    int ans = 1 << 30;
    for (int a : C) {
        for (int b : C) {
            for (int c : C) {
                for (int d : C) {
                    for (int e : C) {
                        int n = a*10000 + b*1000 + c*100 + d*10 + e;
                        
                        if (n >= N) chmin(ans, n);
                        if (n%10000 >= N) chmin(ans, n%10000);
                        if (n%1000 >= N) chmin(ans, n%1000);
                        if (n%100 >= N) chmin(ans, n%100);
                        if (n%10 >= N) chmin(ans, n%10);
                    }
                }
            }
        }
    }

    cout << ans << endl;
}
