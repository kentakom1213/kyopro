//              A - Dial Up
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc125/tasks/arc125_a
// ----------------------------------------

// S <= T でない場合は -1
// それ以外は必ず一致させられる

// 最も近い位置にある0/1を記憶しておく？

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int N, M; cin >> N >> M;
    vector<int> S(N), T(M);
    rep(i, N) cin >> S[i];
    rep(i, M) cin >> T[i];

    bool isOK = true;

    // Tの先頭から検索
    int ans = 0;
    int p = 0;  // a[0]を表すポインタ
    int shortest = N;  // 0<->1を行う最小の操作回数
    for (int t : T) {
        if (S[p] != t)  {
            isOK = false;
            rep(i, shortest) {
                if (S[(p+i)%N] == t) {
                    chmin(shortest, min(p+i, N-(p+i)));
                    p = (i+p) % N;
                    isOK = true;
                    break;
                }
            } 
            ans += shortest;
        }
        ans++;
    }

    cout << (isOK ? ans : -1) << endl;
}
