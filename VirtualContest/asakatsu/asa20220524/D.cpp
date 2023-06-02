// https://atcoder.jp/contests/abc128/tasks/abc128_c

/*
- bit全探索
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
    int N, M;
    cin >> N >> M;
    vector<vector<int>> sw(N);
    rep(i, 0, M) {
        int m; cin >> m;
        rep(j, 0, m) {
            int s; cin >> s;
            s--;
            sw[s].push_back(i);
        }
    }
    vector<int> lamp(M);
    rep(i, 0, M) cin >> lamp[i];

    ll ans = 0;
    rep(i, 0, 1<<N) {
        vector<int> s(M, 0);
        rep(j, 0, N) {
            if ((i>>j) & 1) {
                for (int x : sw[j]) {
                    s[x] ^= 1;
                }
            }
        }
        bool isOK = true;
        rep(i, 0, M) {
            isOK &= lamp[i] == s[i];
        }
        ans += isOK;
    }
    cout << ans << endl;
}