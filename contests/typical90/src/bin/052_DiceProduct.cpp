// 052 - Dice Product（★3）
// -----------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_az
// -----------------------

// 6^100は計算できない
// メモを用意して、更新しながらmodをとっていく
// 1段分の更新に6^2 = 36回
// 最高で100段だから十分間に合う

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int> > Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007; 

template <typename T>
void print_array(vector<vector<T>>& array) {
    int H = array.size();

    cerr << "{" << endl;
    for (int i = 0; i < H; i++) {
        int W = array.at(i).size();
        cerr << "  {";
        for (int j = 0; j < W; j++) {
            if (j < W - 1) cerr << array.at(i).at(j) << ", ";
            else cerr << array.at(i).at(j);
        }
        cerr << "}," << endl;
    }
    cerr << "}" << endl;
}

int main() {
    int N;
    cin >> N;
    initArray(A, N, 6, 0);

    rep(i, N) {
        rep(j, 6) {
            int a; cin >> a;
            A[i][j] = a;
        }
    }
    
    initArray(res, N, 6, 0);
    rep(i, 6) res[0][i] = A[0][i];
    
    // col x --> col y
    rep(i, N-1) {
        rep(x, 6) {
            rep(y, 6) {
                res[i+1][y] += (A[i+1][y] * res[i][x]) % MOD;
            }
        }
    } 
    print_array(res);
    
    ll ans = 0;
    rep(i, 6) {
        ans = (ans + res[N-1][i]) % MOD;
    }
    cout << ans << endl;
}

