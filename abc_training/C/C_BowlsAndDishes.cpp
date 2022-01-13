//          C - Bowls and Dishes
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc190/tasks/abc190_c

// 難しいし慣れない

// AC (解説)
// ----------------------------------------

// 方針
// K <= 16 より 2^K 通りの全探索が間に合う


#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M, K;
    cin >> N >> M;
    vector<pair<int, int>> AB(M), CD(K);
    for (int i = 0; i < M; i++) cin >> AB[i].first >> AB[i].second;
    cin >> K;
    for (int i = 0; i < K; i++) cin >> CD[i].first >> CD[i].second;

    int ans = 0;
    for (int i = 0; i < (1 << K); i++) {
        vector<bool> ball(N, 0);
        for (int j = 0; j < K; j++) {
            if ((i >> j) & 1) ball[ CD[j].second ] = 1;
            else ball[ CD[j].first ] = 1;
        }
        int count = 0;
        for (auto ab : AB) {
            if (ball[ab.first] && ball[ab.second]) count++;
        }
        if (ans < count) ans = count;
    }
    cout << ans << endl;
}