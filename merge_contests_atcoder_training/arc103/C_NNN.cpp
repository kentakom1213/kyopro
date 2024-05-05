//              C - /\/\/\/
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc103/tasks/arc103_a
// 解説
// https://blog.hamayanhamayan.com/entry/2018/09/30/002021

// 方針はあってたけど、ミスなく実装するのが至難の業
// AC (解説)
// ----------------------------------------

// 奇数、偶数のインデックスについて調べる
// 現れる数の個数をmapで管理

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<long long>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int INF = 1 << 30;

int main() {
    int n; cin >> n;
    vector<int> V(n);
    rep(i, n) cin >> V[i];

    map<int, int> even, odd;
    rep(i, n) {
        if (i % 2 == 0) even[V[i]]++;
        else odd[V[i]]++;
    }

    // それぞれのmapについて、(個数, 値)のペアを作成しソート
    vector<vec2> evenp, oddp;
    for (auto [k, v] : even) evenp.push_back(make_pair(v, k));
    for (auto [k, v] : odd) oddp.push_back(make_pair(v, k));

    sort(ALL(evenp), greater<vec2>());
    sort(ALL(oddp), greater<vec2>());

    // どちらも最適
    if (evenp[0].second != oddp[0].second) {
        int ans = n - evenp[0].first - oddp[0].first;
        cout << ans << endl;
        return 0;
    }

    // すべて同じ数
    if (evenp.size() == 1 && oddp.size() == 1) {
        int ans = n / 2;
        cout << ans << endl;
        return 0;
    }

    int ans = INF;
    if (2 <= evenp.size()) chmin(ans, n - evenp[1].first - oddp[0].first);
    if (2 <= oddp.size()) chmin(ans, n - evenp[0].first - oddp[1].first);
    cout << ans << endl;
}
