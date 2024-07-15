
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
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N, M; cin >> N >> M;
    vector<ll> A(N), S(N+1);
    rep(i, N) cin >> A[i];

    rep(i, N) {
        S[i+1] = S[i] + A[i];
    }

    rep(l, N-M) {
        // 二分探索
        auto range_sum = [&](int x) -> bool {
            
        };

        int r = N;
        while (r - l > 1) {

        }
    }
}