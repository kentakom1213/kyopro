//               A - Make 10
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc126/tasks/arc126_a
// ----------------------------------------

// 2, 3, 4の和から10をつくる組合せ
// 10 = 2+2+2+2+2
//    = 2+2+2+4
//    = 2+2+3+3
//    = 2+4+4
//    = 3+3+4


#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

ll make10(ll n2, ll n3, ll n4) {

}

int main() {
    int T; cin >> T;
    vector<ll> res(T);
    rep(i, T) {
        ll n2, n3, n4; cin >> n2 >> n3 >> n4;
        res[i] = make10(n2, n3, n4);
    }

    rep(i, T) cout << res[i] << endl;
}