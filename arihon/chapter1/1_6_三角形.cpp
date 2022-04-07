/*
# 三角形
## 制約
- $3 \le n \le 100$
- $1 \le a_i \le 10^6$
*/

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
    int N; cin >> N;
    vector<int> A(N);
    rep(i, N) cin >> A[i];

    // 三角形が作れるかどうか
    auto can_make_triangle = [](ll a, ll b, ll c) -> bool {
        return a+b > c && b+c > a && c+a > b;
    };

    // O(n^3) でも間に合う
    ll max_circ = 0;
    range(i, 0, N) {
        range(j, i+1, N) {
            range(k, j+1, N) {
                ll a = A[i], b = A[j], c = A[k];
                if (can_make_triangle(a, b, c)) {
                    chmax(max_circ, a+b+c);
                }
            }
        }
    }

    cout << max_circ << endl;
}