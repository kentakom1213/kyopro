//         D - Destroyer Takahashi
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc230/tasks/abc230_d
// ----------------------------------------

// 区間スケジューリングではないっぽい

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<long long>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    ll N, D; cin >> N >> D;
    vector<vec2> walls(N);
    rep(i, N) cin >> walls[i].first >> walls[i].second;

    // 左端でソート
    sort(ALL(walls));

    for (auto [l, r] : walls) {
        cerr << "{" << l << "," << r << "}, ";
    }
    cerr << endl;

    // 貪欲に取る
    ll cnt = 0;
    ll i = 0, left, right;
    while (i < N) {
        left = walls[i].second;
        right = left + D;

        // 壊せる壁を全て調べる
        while (i < N && walls[i].first < right) i++;

        cerr << left << " " << right << endl;
        cnt++;
    }

    cout << cnt << endl;
}
