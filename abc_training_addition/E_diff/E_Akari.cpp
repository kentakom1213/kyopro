// E - Akari
// ---------
// 問題
// https://atcoder.jp/contests/abc182/tasks/abc182_e
// ---------

// 愚直にシミュレートすると、O(NHW);
// シミュレートの仕方を工夫してみよう

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int> > Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int H, W, N, M;
    cin >> H >> W >> N >> M;
    vector<vec2> Light(N), Block(M);
    rep(i, N) {
        int a, b;
        cin >> a >> b;
        a--, b--;
        Light[i] = make_pair(a, b);
    }
    rep(i, M) {
        int c, d;
        cin >> c >> d;
        c--, d--;
        Block[i] = make_pair(c, d);
    }

    
}
