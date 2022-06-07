// https://atcoder.jp/contests/abc146/tasks/abc146_d

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

set<int> JOIN(vector<int> &A, vector<int> &B) {
    int la=A.size(), lb=B.size();
    set<int> res;
    for(int v: A) res.insert(v);
    for(int v: B) res.insert(v);
    return res;
}

using pii = pair<int, int>;
vector<vector<int>> G;

int main() {
    int N; cin >> N;
    G.assign(N, {});
    vector<pii> edges(N-1);
    for (auto &[a, b] : edges) {
        cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    // 辺を順に処理
    vector<vector<int>> vcolor(N);
    vector<int> ans(N-1);
    int max_col = 0;
    rep(i, 0, N-1) {
        auto [a, b] = edges[i];
        set<int> used = JOIN(vcolor[a], vcolor[b]);

        // 塗れる色を見つける
        int col = 0;
        rep(j, 1, used.size()+2) {
            if (used.find(j) == used.end()) {
                col = j;
                break;
            }
        }

        ans[i] = col;
        vcolor[a].push_back(col);
        vcolor[b].push_back(col);
        chmax(max_col, col);
    }

    cout << max_col << endl;
    for (int v : ans) cout << v << endl;
}
