#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

template <typename T>
void print_array(vector<vector<T>>& array) {
    int H = array.size();

    cerr << "{" << endl;
    for (int i = 0; i < H; i++) {
        int W = array.at(i).size();
        cerr << "  {";
        for (int j = 0; j < W; j++) {
            if (j < W - 1) cerr << array.at(i).at(j) << ", ";
            else cerr << array.at(i).at(j);
        }
        cerr << "}," << endl;
    }
    cerr << "}" << endl;
}

void dfs(const vector<vector<int>> &G, int cur, vector<vector<int>> &memo, vector<int> &is_visited, const vector<int> &X) {
    is_visited[cur] = 1;
    if (G[cur].size() == 0) {
        memo[cur][0] = X[cur];
    } else {
        vector<int> merged(20, -1);
        merged.push_back(X[cur]);
        for (int nxt : G[cur]) {
            if (is_visited[nxt]) continue;
            dfs(G, nxt, memo, is_visited, X);
            for (int v : memo[nxt]) merged.push_back(v);
        }
        sort(merged.begin(), merged.end(), greater<int>());
        rep(i, 20) memo[cur][i] = merged[i];
    }
}

int main() {
    int N, Q; cin >> N >> Q;
    vector<int> X(N); rep(i, N) cin >> X[i];
    vector<vector<int>> G(N);
    rep(i, N-1) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    initArray(memo, N, 20, -1);
    vector<int> is_visited(N, 0);


    // dfs
    dfs(G, 0, memo, is_visited, X);

    rep(i, Q) {
        int v, k; cin >> v >> k;
        v--, k--;
        cout << memo[v][k] << endl;
    }
}