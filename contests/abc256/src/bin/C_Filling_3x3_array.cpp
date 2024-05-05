//          C - Filling 3x3 array          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc256/tasks/abc256_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int H[3], W[3];
int ans;

void dfs(int pos, Array<int> arr) {
    int r = pos / 3;
    int c = pos % 3;

    if (c == 2) {
        arr[r][c] = H[r] - arr[r][0] - arr[r][1];
        dfs(pos+1, arr);
    }
    else if (r == 2) {
        rep(j, 0, 3) {
            arr[r][j] = W[j] - arr[0][j] - arr[1][j];
        }
        bool isOK = arr[2][0] + arr[2][1] + arr[2][2] == H[2];
        rep(i, 0, 3) rep(j, 0, 3) isOK &= arr[i][j] > 0;
        ans += isOK;
    }
    else {
        int cell_max = min(H[r]-2, W[c]-2);
        rep(i, 1, cell_max+1) {
            arr[r][c] = i;
            dfs(pos+1, arr);
        }
    }
}

int main() {
    cin >> H[0] >> H[1] >> H[2];
    cin >> W[0] >> W[1] >> W[2];
    Array<int> arr = {{0, 0, 0}, {0, 0, 0}, {0, 0, 0}};
    dfs(0, arr);
    cout << ans << endl;
}
