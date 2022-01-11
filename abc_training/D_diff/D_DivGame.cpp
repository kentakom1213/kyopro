//              D - Div Game
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc169/tasks/abc169_d

// 整数型には気をつけよう

// AC
// ----------------------------------------

// 素因数分解 -> 約数をえらぶの流れかな

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

typedef long long ll;
vector<pair<ll, int>> factoring_pair(ll n) {
    vector<pair<ll, int>> res;
    for (ll i = 2; i*i <= n; i++) {
        int count = 0;
        while (n % i == 0) {
            count++;
            n /= i;
        }
        if (count) res.push_back({i, count});
    }
    if (n != 1) res.push_back({n, 1});
    return res;
}

int main() {
    ll N; cin >> N;
    auto factors = factoring_pair(N);

    // 素因数の個数について、満たす最大の三角数を二分探索
    int res = 0;
    for (auto [p, n] : factors) {

        auto isOK = [&](int x, int n) {
            return x * (x + 1) / 2 <= n;
            };

        int l = 0, r = n;
        while (r - l > 1) {
            int mid = (l + r) / 2;
            if (isOK(mid, n)) l = mid;
            else r = mid;
        }

        // printf("p:%lld, n:%d, triangle:%d\n", p, n, l);
        res += max(l, 1);
    }

    cout << res << endl;
}