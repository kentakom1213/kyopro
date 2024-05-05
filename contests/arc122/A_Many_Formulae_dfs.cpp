//           A - Many Formulae
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc122/tasks/arc122_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr ll MOD = 1000000007;

int N;
vector<ll> A;
ll ans;

// {"+" -> 0, "-" -> 1}
void dfs(int op, int ind, ll sum) {
    if (ind+1 == N) {
        if (op == 0) {
            ans += sum + A[ind];
            ans %= MOD;
        } else {
            ans += sum - A[ind];
        }
    } else if (op == 0) {
        dfs(0, ind+1, sum + A[ind]);
        dfs(1, ind+1, sum + A[ind]);
    } else {
        dfs(0, ind+1, sum - A[ind]);
    }
}

int main() {
    cin >> N;
    A.assign(N, 0);
    rep(i, N) cin >> A[i];

    ans = 0;
    dfs(0, 0, 0);
    cout << ans % MOD << endl;
}
