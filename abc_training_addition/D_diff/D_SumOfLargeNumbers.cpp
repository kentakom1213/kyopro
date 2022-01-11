//        D - Sum of Large Numbers
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc163/tasks/abc163_d

// 参考
// https://drken1215.hatenablog.com/entry/2020/04/20/003900

// AC
// ----------------------------------------

// 10^100のおかげでa個選ぶ場合とb個選ぶ場合の区別をしなくて良くなった

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
const ll MOD = 1000000007;

int main() {
    int N, K; cin >> N >> K;

    auto sum_a2b = [](ll a, ll b) -> ll {
        return (a + b) * (b - a + 1) / 2;
    };

    ll res = 1; // 全て選ぶ
    for (ll n = K; n <= N; n++) {
        ll mini = sum_a2b(0, n-1);
        ll maxi = sum_a2b(N-n+1, N);
        // printf("mini:%lld, maxi:%lld\n", mini, maxi);
        res = (res + maxi - mini + 1) % MOD;
    }

    cout << res << endl;
}