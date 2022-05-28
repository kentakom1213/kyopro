// https://atcoder.jp/contests/abc241/tasks/abc241_e

/*
- ループ検出
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    ll N, K;
    cin >> N >> K;
    vector<ll> A(N);
    rep(i, 0, N) cin >> A[i];

    map<int, int> memo;

    ll i=0, x=0, loop=0;
    while (i<K) {
        x += A[x % N];
        if (memo.find(x) == memo.end()) memo[x] = i;
        else loop = i - memo[x];
        i++;
    }

    // 経路の短縮
    K %= loop;

    i=0, x=0;
    while (i<K) {
        x += A[x % N];
        i++;
    }

    cout << x << endl;
}