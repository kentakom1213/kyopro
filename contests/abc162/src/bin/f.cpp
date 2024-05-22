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

int N;
vector<ll> A;

// nが偶数の場合
void solve_even() {
    ll Se = 0, So = 0;

    rep(i, 0, N) {
        if (i % 2 == 0) {
            Se += A[i];
        } else {
            So += A[i];
        }
    }

    cout << max(Se, So) << endl;
}

int main(){
    cin >> N;
    A.assign(N, 0);
    for (auto &a : A) {
        cin >> a;
    }

    if (N % 2 == 0) {
        solve_even();
        return 0;
    }

    // dp[i][j] := A[i]を採用するとき，合計で`⌈i/2⌉-j`個選んだことになる場合の最大値
    
    return 0;
}
