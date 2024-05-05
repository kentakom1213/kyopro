//            B - 解像度が低い。
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc017/tasks/arc017_2
// ----------------------------------------

// 普通に調べると、O(NK)
// 連続した部分列であるから、尺取り法で実装したい


#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int N, K; cin >> N >> K;
    vector<ll> A(N, 0);
    rep(i, N) cin >> A[i];

    // 尺取り法
    deque<ll> q;
    int r = 0;
    rep(l, N) {
        while ()
    }
}