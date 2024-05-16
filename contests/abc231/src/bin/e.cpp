#include <bits/stdc++.h>

using namespace std;
using ll = long long;
using pll = pair<ll, ll>;

int N;
ll X;
vector<ll> A;
map<pll, ll> memo;

ll rec(ll x, ll i) {
    // このコインで払い切る場合
    if (i + 1 == N) {
        return x / A[i];
    }

    if (memo.find({x, i}) != memo.end()) {
        return memo[{x, i}];
    }

    // 今回払う金額
    ll pay = x % A[i + 1];

    // ちょうどを支払う場合
    ll p1 = pay / A[i];
    ll n1 = rec(x / A[i + 1] * A[i + 1], i + 1);

    // 多めに支払い，お釣りをもらう場合
    ll p2 = (A[i + 1] - pay) / A[i];
    ll n2 = rec((x + A[i + 1] - 1) / A[i + 1] * A[i + 1], i + 1);

    ll res = min(p1 + n1, p2 + n2);
    memo[{x, i}] = res;
    return res;
}

int main() {
    cin >> N >> X;
    A.assign(N, 0);
    for (auto &x : A) {
        cin >> x;
    }

    // ===== 上位桁から順に決定
    ll ans = rec(X, 0);

    cout << ans << endl;
}
