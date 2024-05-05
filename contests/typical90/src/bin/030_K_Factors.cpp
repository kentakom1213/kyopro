//          030 - K Factors（★5）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ad
// ----------------------------------------

// ふるいで処理

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll N, K; cin >> N >> K;

    vector<ll> sieve(N+1, 0);
    for (int i = 2; i <= N; i++) {
        if (sieve[i]) continue;  // 素数でないものは無視
        for (int j = 1; i*j <= N; j++) {
            sieve[i*j]++;
        }
    }

    ll res = 0;
    for (auto n : sieve) res += n >= K;

    cout << res << endl;
}
