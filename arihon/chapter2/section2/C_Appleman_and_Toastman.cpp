//         C. Appleman and Toastman
// ----------------------------------------
// 問題
// https://codeforces.com/contest/462/problem/C

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

typedef priority_queue<ll, vector<ll> > min_heap;

int main() {
    int N; cin >> N;

    ll ans = 0;
    min_heap pq;
    rep(i, N) {
        ll v; cin >> v;
        pq.push(v);
        ans += v;
    }

    rep(i, N-1) {
        ll min1st = pq.top(); pq.pop();
        ll min2nd = pq.top(); pq.pop();
        ans += min1st + min2nd;
        pq.push(min1st + min2nd);
    }

    cout << ans << endl;
}
