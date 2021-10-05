//        C - Energy Drink Collector
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc121/tasks/abc121_c

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll N, M; cin >> N >> M;
    vector<pair<ll, ll>> shop(N);
    for (auto& [a, b] : shop) cin >> a >> b;
    sort(shop.begin(), shop.end());

    ll res = 0;
    for (int i = 0; i < N; i++) {
        if (M > shop[i].second) {
            M -= shop[i].second;
            res += shop[i].second * shop[i].first;
        } else {
            res += shop[i].first * M;
            break;
        }
    }
    cout << res << endl;
}