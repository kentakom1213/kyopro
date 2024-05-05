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

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int N;
vector<int> B, salary;

void dfs(int cur) {
    bool has_buka = false;
    int min_salary=1<<30, max_salaly=1;

    rep(i, N) if (i!=cur) {
        if (cur == B[i]) {
            has_buka = true;
            dfs(i);
            chmin(min_salary, salary[i]);
            chmax(max_salaly, salary[i]);
        }
    }

    if (has_buka) {
        salary[cur] = min_salary + max_salaly + 1;
    } else {
        salary[cur] = 1;
    }
}

int main() {
    cin >> N;
    B.assign(N, 0);
    salary.assign(N, 0);

    range(i, 1, N) {
        cin >> B[i];
        B[i]--;
    }

    dfs(0);

    cout << salary[0] << endl;
}
