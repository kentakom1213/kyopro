//               B - 完全数
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc026/tasks/arc026_2
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

// 約数列挙
ll factoring_and_sum(ll n) {
    ll res = -n;
    for (ll i = 1; i*i<=n; i++) {
        if (i*i == n) {
            res += i;
        } else if (n % i == 0) {
            res += i;
            res += n / i;
        }
    }
    return res;
}

int main() {
    ll N; cin >> N;
    ll diff = factoring_and_sum(N) - N;
    if (diff < 0) {
        cout << "Deficient" << endl;
    } else if (diff == 0) {
        cout << "Perfect" << endl;
    } else {
        cout << "Abundant" << endl;
    }
}
