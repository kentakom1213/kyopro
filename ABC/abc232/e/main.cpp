
// O(K)であれば間に合う


#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 998244353;


int main() {
    ll H, W, K; cin >> H >> W >> K;
    ll x1, y1, x2, y2; cin >> x1 >> y1 >> x2 >> y2;

    // 変化量
    ll dx = abs(x1 - x2);
    ll dy = abs(y1 - y2);

    // dx, dyの選び方を試す
    // cx : choose x
    for (int cx = -K; cx < K; cx++) {
        ll cy = K - cx;
        
    }

}