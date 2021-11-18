//           D - Water Heater
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc183/tasks/abc183_d

// AC
// ----------------------------------------

// シミュレートしていく感じ？
// 区間への加算 -> いもす法でもいいかも

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

const ll MAX_SIZE = 200100;

int main() {
    ll N, W; cin >> N >> W;
    vector<ll> S(N), T(N), P(N);
    rep(i, N) cin >> S[i] >> T[i] >> P[i];

    vector<ll> imos(MAX_SIZE, 0);
    rep(i, N) {
        imos[S[i]] += P[i];
        imos[T[i]] -= P[i];
    }

    // 累積和
    vector<ll> water(MAX_SIZE, 0);
    water[0] = imos[0];
    for (int i = 1; i < N; i++) {
        water[i] = water[i-1] + imos[i];
    }

    bool isOK = true;
    rep(i, N) {
        if (water[i] > W) isOK = false;
    }
    cout << (isOK ? "Yes" : "No") << endl;
}