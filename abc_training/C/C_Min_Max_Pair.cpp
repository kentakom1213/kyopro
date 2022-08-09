//             C - Min Max Pair            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc262/tasks/abc262_c
// ----------------------------------------

/*
A[i] == i となるiの組合せ
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N; cin >> N;
    vector<int> A(N);
    rep(i, 0, N) {
        cin >> A[i];
        A[i]--;
    }

    ll id = 0;  // A[i] == i
    ll couple = 0;
    rep(i, 0, N) {
        if (A[i] == i) id++;
        else {
            if (A[A[i]] == i) couple++;
        }
    }

    ll ans = (id * (id-1) / 2) + (couple / 2);
    cout << ans << endl;
}
