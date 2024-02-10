// E - This Message Will Self-Destruct in 5s
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc166/tasks/abc166_e

// AC
// ----------------------------------------

//     j-i = A[i]+A[j]
// <=> i+A[i] = j-A[j]
// i-A[i]の情報をmapに保存してから探索
// O(n)

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, n, m) for (int i = (int)n; i < (int)m; i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int N; cin >> N;
    vector<int> A(N);
    rep(i, N) cin >> A[i];

    // 値とインデックスの情報をmapに保存
    map<int, int> diffs;
    rep(i, N) {
        diffs[i-A[i]]++;
    }

    ll cnt = 0;
    rep(i, N) {
        cnt += diffs[i+A[i]];
    }

    cout << cnt << endl;
}
