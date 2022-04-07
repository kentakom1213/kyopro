/*
# くじ引き

- $O(n^4)$ の解法
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int> > name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N, M; cin >> N >> M;
    vector<int> K(N);
    rep(i, N) cin >> K[i];

    // 4重ループ
    rep(i, N) rep(j, N) rep(k, N) rep(l, N) {
        if (K[i] + K[j] + K[k] + K[l] == M) {
            cout << "Yes" << endl;
            return 0;
        }
    }
    cout << "No" << endl;
}