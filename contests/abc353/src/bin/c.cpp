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

ll M = 100000000;
int N;
ll A[303030];

int main(){
    cin >> N;
    rep(i, 0, N) {
        cin >> A[i];
    }

    // 総和を求める
    ll sum = 0;

    rep(i, 0, N) {
        sum += A[i] * (N - 1);
    }

    cerr << "sum: " << sum << endl;

    // ペアの和で，Mを超えるものをカウント
    sort(A, A + N);

    // 尺取り法
    ll cnt = 0;  // 条件を満たすペアの個数
    int r = N;

    rep(i, 0, N) {
        r = max(r, i + 1);
        while (r - 1 > i && A[r - 1] + A[i] >= M) {
            r--;
        }
        cnt += N - r;
    }

    cerr << "cnt: " << cnt << endl;

    sum -= cnt * M;

    cout << sum << endl;
    
    return 0;
}
