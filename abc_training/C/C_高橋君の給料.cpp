//                C - 高橋君の給料               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc026/tasks/abc026_c
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
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

void dfs(vector<int> &salary, const vector<int> B, int cur) {
    bool has_buka = false;
    int min_buka = 1, max_buka = 1;
    for (auto b : B) {
        if (b == cur) {
            has_buka = true;
            if (salary[b] == 0) dfs(salary, B, b);

            // 最小値、最大値の更新
            chmin(min_buka, salary[b]);
            chmax(max_buka, salary[b]);
        }
    }
    if (has_buka) {
        salary[cur] = min_buka + max_buka + 1;
    } else {
        salary[cur] = 1;
    }
}

int main() {
    int N; cin >> N;
    vector<int> B(N), salary(N, 0);
    rep(i, N-1) {
        cin >> B[i+1];
        B[i]--;
    }

    // 木DP
    dfs(salary, B, 0);

    cout << salary[0] << endl;
}
