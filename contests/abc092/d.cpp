#include <bits/stdc++.h>
// #include <atcoder/all>
using namespace std;
// using namespace atcoder;
#define rep(i, a, n) for(int i = a; i < n; i++)
#define rrep(i, a, n) for(int i = a; i >= n; i--)
#define inr(l, x, r) (l <= x && x < r)
#define ll long long
#define ld long double

// using mint = modint1000000007;
// using mint = modint998244353;
constexpr int IINF = 1001001001;
constexpr ll INF = 9e18;

template<class t,class u> void chmax(t&a,u b){if(a<b)a=b;}
template<class t,class u> void chmin(t&a,u b){if(b<a)a=b;}

constexpr int K = 100;
bool F[100][100];

int main(){
    int a, b;
    cin >> a >> b;

    // 上半分を黒で埋める
    rep(i, 0, K / 2) {
        rep(j, 0, K) {
            F[i][j] = true;
        }
    }

    // A-1個の白を配置
    rep(i, 0, a - 1) {
        int r = i / 50, c = i % 50;
        F[2 * r][2 * c] = false;
    }

    rep(i, 0, b - 1) {
        int r = i / 50, c = i % 50;
        F[2 * r + 51][2 * c] = true;
    }

    // 出力
    cout << K << " " << K << endl;

    rep(i, 0, K) {
        rep(j, 0, K) {
            cout << (F[i][j] ? '#' : '.');
        }
        cout << endl;
    }
    
    return 0;
}
