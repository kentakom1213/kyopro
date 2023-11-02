// https://atcoder.jp/contests/abc240/tasks/abc240_e

// WA

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
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

vector<vector<int>> G;
vector<int> left, right;
int idx = 0;

void dfs(int prev, int cur, vector<int> &left, vector<int> &right) {
    bool is_leaf = cur != 0 && G[cur].size() == 1;

    if (is_leaf) {
        left[cur] = left[prev];
        right[cur] = left[cur];

        printf("cur: %d, prev: %d\n", cur, prev);
        print_vector(left);
        print_vector(right);
        cout << "\n";
    }
    else {
        left[cur] = left[prev];

        for (int nxt : G[cur]) {
            if (nxt == prev) continue;
            dfs(cur, nxt, left, right);
            right[cur] = right[nxt] + 1;
        }

        printf("cur: %d, prev: %d\n", cur, prev);
        print_vector(left);
        print_vector(right);
        cout << "\n";
    }
}

int main() {
    ll N; cin >> N;
    G.assign(N, {});
    for (int i=0; i<N-1; i++) {
        int u, v; cin >> u >> v;
        u--, v--;
        G[u].push_back(v);
        G[v].push_back(u);
    }

    left.assign(N, -1);
    right.assign(N, -1);
    left[0] = 1;
    dfs(0, 0, left, right);

    // for (int i = 0; i < N; i++) {
    //     printf("%d %d\n", left[i], right[i]);
    // }
}