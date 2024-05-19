#include <bits/stdc++.h>
#include <atcoder/all>
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

void solve() {
    int N;
    cin >> N;
    vector<ll> A(N);
    for (auto &v : A) {
        cin >> v;
    }

    // LISをもとめる
    vector<ll> dp(N, INF);
    rep(i, 0, N) {
        int idx = lower_bound(dp.begin(), dp.end(), A[i]) - dp.begin();

    }
}

int main(){
    int T;
    cin >> T;

    while (T--) {
        solve();
    }
    
    return 0;
}
