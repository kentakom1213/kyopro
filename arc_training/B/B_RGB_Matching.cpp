//             B - RGB Matching            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc121/tasks/arc121_b
// ----------------------------------------

/*
## 方針
- 全探索すると $O(2N!!)$
- 色ごとにマッチングする
- 残りを差が小さい順にとっていく
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

typedef pair<ll, char> P;
int N, rcnt, gcnt, bcnt;

int main() {
    cin >> N;
    set<ll> r, g, b;
    rep(i, 2*N) {
        int a; cin >> a;
        char c; cin >> c;
        if (c=='R') {
            r.insert(a);
            rcnt++;
        }
        if (c=='G') {
            g.insert(a);
            gcnt++;
        }
        if (c=='B') {
            b.insert(a);
            bcnt++;
        }
    }

    if (rcnt%2==0 && gcnt%2==0 && bcnt%2==0) {  // すべて偶数
        cout << 0 << endl;
        return 0;
    }

    ll min_diff = 1LL << 50;
    auto r_itr=r.begin(), g_itr=g.begin(), b_itr=b.begin();
    if (rcnt % 2 == 0) {  // G, Bでマッチ
        while (g_itr!=g.end() && b_itr!=b.end()) {
            if (g_itr!=g.end() && *g_itr <= *b_itr) g_itr++;
            else if (b_itr!=b.end()) b_itr++;
            chmin(min_diff, abs(*g_itr - *b_itr));
        }
    }
    if (gcnt % 2 == 0) {  // R, Bでマッチ
        while (r_itr!=r.end() && b_itr!=b.end()) {
            if (r_itr!=r.end() && *r_itr <= *b_itr) r_itr++;
            else if (b_itr!=b.end()) b_itr++;
            chmin(min_diff, abs(*r_itr - *b_itr));
        }
    }
    if (bcnt % 2 == 0) {  // R, Gでマッチ
        while (g_itr!=g.end() && r_itr!=r.end()) {
            if (g_itr!=g.end() && *g_itr <= *r_itr) g_itr++;
            else if (r_itr!=r.end()) r_itr++;
            chmin(min_diff, abs(*g_itr - *r_itr));
        }
    }

    cout << min_diff << endl;
}
