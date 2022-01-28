//         B - Counting of Trees
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/nikkei2019-2-qual/tasks/nikkei2019_2_qual_b
// ----------------------------------------

// 木の数え上げ
// 高さ順にソート
// 貪欲に一本ずつ辺を張っていく

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = begin; i < (int)end; i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<long long>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 998244353;

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int main() {
    ll N; cin >> N;
    vector<ll> D(N);
    rep(i, N) cin >> D[i];

    // カウントしておく
    map<ll, ll> cnt_V;
    rep(i, N) cnt_V[D[i]]++;

    ll max_depth = cnt_V.rbegin()->first;

    // depth[i] := 高さNの辺の数 (% MOD)
    vector<ll> depth(max_depth+1, 0);
    depth[0] = 1;

    range(i, 1, max_depth+1) {
        depth[i] = depth[i-1] * cnt_V[i] % MOD;
        print_vector(depth);
    }
}
