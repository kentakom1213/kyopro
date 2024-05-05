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

int N;

ll min_diff(vector<ll> fst, vector<ll> sec) {
    ll ans = 1LL << 50;
    int f=0, s=0;
    while (f < fst.size() && s < sec.size()) {
        chmin(ans, abs(fst[f] - sec[s]));
        if (f<fst.size() && fst[f] <= sec[s]) f++;
        else if (fst[f] > sec[s]) s++;
    }
    return ans;
}

int main() {
    cin >> N;
    vector<ll> r, g, b;
    rep(i, 2*N) {
        ll a; cin >> a;
        char c; cin >> c;
        switch (c) {
            break; case 'R':
                r.push_back(a);
            break; case 'G':
                g.push_back(a);
            break; case 'B':
                b.push_back(a);
        }
    }
    sort(ALL(r));
    sort(ALL(g));
    sort(ALL(b));

    ll gb = min_diff(g, b);
    ll rg = min_diff(r, g);
    ll rb = min_diff(r, b);

    if (r.size()%2==0 && g.size()%2==0 && b.size()%2==0) {
        cout << 0 << endl;
        return 0;
    }
    if (r.size()%2 == 0) {
        cout << min(gb, rg+rb) << endl;
        return 0;
    }
    if (g.size()%2 == 0) {
        cout << min(rb, gb+rg) << endl;
        return 0;
    }
    if (b.size()%2 == 0) {
        cout << min(rg, gb+rb) << endl;
        return 0;
    }
}
