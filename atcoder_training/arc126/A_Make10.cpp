//               A - Make 10
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc126/tasks/arc126_a
// ----------------------------------------

// T <= 100

// 3は必ずペアにして使う必要がある

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

int main() {
    int T; cin >> T;
    while(T--) {
        ll n2, n3, n4, ans; cin >> n2 >> n3 >> n4;
        ll rem_n4 = n4 - n3/2;
        ll rem_n2 = n2 + rem_n4;
        if (rem_n2 <= 0 || rem_n4 <= 0) {
            cout << n4 + n2/2 << endl;
        } else {
            
        }
    }

}