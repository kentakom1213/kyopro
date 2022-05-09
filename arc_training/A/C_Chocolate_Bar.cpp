//            C - Chocolate Bar            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc062/tasks/arc074_a
// ----------------------------------------

/*
(1)
 ---------
|   r1    |
|---------|
|    |    |
| r2 | r3 |
 ---------

 ---------
|   r1    |
|---------|
|   r2    |
|---------|
|   r3    |
 ---------

(2)
 ---------
|   |     |
|   | r2  |
|r1 |-----|
|   | r3  |
 ---------

 -----------
|   |   |   |
|   |   |   |
|r1 |r2 |r3 |
|   |   |   |
 -----------
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int solve(int h, int w) {
    if (h%3==0 || w%3==0) return 0;

    ll ans = (ll)1e20;

    // (1)
    ll w1=w/2, w2=w-w1;
    for (ll i=0; i < h; i++) {
        // 1. 
        ll r1=i*w, r2=(h-i)*w1, r3=(h-i)*w2;
        ll Smax = max(r1, max(r2, r3)), Smin = min(r1, min(r2, r3));
        chmin(ans, Smax - Smin);

        // 2.
        ll h1=(h-i)/2, h2=h-i-h1;
        r1=i*w, r2=h1*w, r3=h2*w;
        Smax = max(r1, max(r2, r3)), Smin = min(r1, min(r2, r3));
        chmin(ans, Smax - Smin);
    }

    // (2)
    ll h1=h/2, h2=h-h1;
    for (ll i=0; i < w; i++) {
        // 1.
        ll r1=h*i, r2=h1*(w-i), r3=h2*(w-i);
        ll Smax = max(r1, max(r2, r3)), Smin = min(r1, min(r2, r3));
        chmin(ans, Smax - Smin);

        // 2.
        ll w1=(w-i)/2, w2=w-i-w1;
        r1=h*i, r2=h*w1, r3=h*w2;
        Smax = max(r1, max(r2, r3)), Smin = min(r1, min(r2, r3));
        chmin(ans, Smax - Smin);
    }

    return ans;
}

int main() {
    ll h, w;
    cin >> h >> w;

    cout << solve(h, w) << endl;
}
