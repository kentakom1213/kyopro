//          029 - Long Bricks（★5）          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ac
// ----------------------------------------

/*

## 方針
- セグ木（max）を使う
- 区間取得、一点更新

## 参考
https://betrue12.hateblo.jp/entry/2020/09/23/005940

遅延セグ木わからん

*/

#include <bits/stdc++.h>
#include <atcoder/lazysegtree>

using namespace std;
using namespace atcoder;

#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

// モノイド
using S = ll;
using F = ll;

S op(S a, S b) {
    return max(a, b);
}

S e() {
    return 0;
}

S mapping(F f, S a) {
    return max(f, a);
}

S composition(F f, F g) {
    return max(f, g);
}

F id() {
    return 0;
}

int main() {
    int W, N; cin >> W >> N;
    vector<pll> LR(N);
    for (auto &[l, r] : LR) {
        cin >> l >> r;
        l--;
    }

    vector<ll> v(W, 0);
    lazy_segtree<S, op, e, F, mapping, composition, id> seg(v);

    vector<ll> ans(N, 0);

    for (int i = 0; i < N; i++) {
        auto [l, r] = LR[i];
        ll maxi = seg.prod(l, r) + 1;
        seg.apply(l, r, maxi);
        ans[i] = maxi;
    }

    for (auto v : ans) {
        cout << v << endl;
    }
}
