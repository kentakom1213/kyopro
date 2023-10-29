//                C - Peaks                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc166/tasks/abc166_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int N, M, H[101010];
bool isGood[101010];

int main() {
    cin >> N >> M;
    rep(i, 0, N) cin >> H[i];

    FILL(isGood, true);

    // 順に比較して，大きい方を残す
    rep(i, 0, M) {
        int a, b; cin >> a >> b;
        a--, b--;
        isGood[a] &= H[a] > H[b];
        isGood[b] &= H[a] < H[b];
    }

    int ans = 0;
    rep(i, 0, N) ans += isGood[i];
    cout << ans << endl;
}