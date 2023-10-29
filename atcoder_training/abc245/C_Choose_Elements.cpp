//           C - Choose Elements           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc245/tasks/abc245_c
// ----------------------------------------

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

template<class T>
void print_set(set<T> S) {
    cout << "{ ";
    for (auto val : S) {
        cout << val << ", "[val == *(--S.end())];
    }
    cout << "}" << endl;
}

int main() {
    int N, K; cin >> N >> K;
    vector<ll> A(N), B(N);
    rep(i, 0, N) cin >> A[i];
    rep(i, 0, N) cin >> B[i];

    set<ll> ok[] = {{}, {A[0], B[0]}};
    rep(i, 1, N) {
        ok[~i&1].clear();
        set<ll> cur=ok[i&1], nxt={A[i], B[i]};
        for (ll x : cur) for (ll y : nxt) {
            if (abs(x - y) <= K) ok[~i&1].insert(y);
        }
    }
    if (ok[N&1].empty()) cout << "No" << endl;
    else cout << "Yes" << endl;
}
