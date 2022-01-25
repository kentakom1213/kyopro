//              C - /\/\/\/
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc103/tasks/arc103_a

// WA
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
    vector<vec2> nums;
    for (auto [k, v] : even) nums.push_back(make_pair(v, k));
    for (auto [k, v] : odd) nums.push_back(make_pair(v, -k));

    sort(ALL(nums), greater<vec2>());

    // kが異なるものを異なるものを貪欲に選ぶ
    // kはoddのものを負にしてある
    vec2 evens = make_pair(-1, 0), odds = make_pair(-1, 0);
    for (auto [v, k] : nums) {
        if (k > 0 && k != odds.first) evens = make_pair(k, v);
        if (k < 0 && -k != evens.first) odds = make_pair(-k, v);
    }

    // 良い感じに処理
    cout << n - evens.second - odds.second << endl;
}
