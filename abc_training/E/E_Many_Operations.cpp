//           E - Many Operations           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc261/tasks/abc261_e
// ----------------------------------------

/*
# 方針
bit単位で関数の合成を行う

f_n = op_n . (f_n-1 . f_n-1)

であることを利用する。
ただし、f_0 は恒等写像（関数合成の単位元）
*/

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

#define bit(x,i) (((x)>>(i))&1)

int main() {
    ll N, C; cin >> N >> C;
    vector<pll> op(N);
    for (auto &[t, a] : op) cin >> t >> a;

    vector<int> ans(N);
    for (int k = 0; k < 30; k++) {
        array<int, 2> func = {0, 1};
        int crr = bit(C, k);

        for (int i = 0; i < N; i++) {
            array<int, 2> f;
            int x = bit(op[i].second, k);

            // 関数の合成
            switch (op[i].first) {
                break; case 1: f = {0&x, 1&x};
                break; case 2: f = {0|x, 1|x};
                break; case 3: f = {0^x, 1^x};
            }
            func = {f[func[0]], f[func[1]]};

            // 答えに追加
            crr = func[crr];
            ans[i] |= crr << k;
        }
    }

    for (int i = 0; i < N; i++) cout << ans[i] << endl;
}
