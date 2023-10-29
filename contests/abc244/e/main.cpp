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
constexpr int INF = 1<<30;
constexpr int MOD = 998244353;

int N, M, K, S, T, X;
vector<vector<int>> G;

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

void bfs(int cur, vector<int> &dist) {
    queue<vec2> q;
    q.push(make_pair(cur, 0));

    while (!q.empty()) {
        auto [cur, d] = q.front(); q.pop();
        if (cur == X) d *= -1;
        for (int nxt : G[cur]) {
            if (dist[nxt] < INF) continue;
            if (d < 0) {
                dist[nxt] = d-1;
                q.push(make_pair(nxt, d-1));
            } else {
                dist[nxt] = d+1;
                q.push(make_pair(nxt, d+1));
            }
            if (nxt == X) dist[nxt] *= -1;
        }
    }
}

int main() {
    cin >> N >> M >> K >> S >> T >> X;
    S--, T--, X--;

    G.assign(N, vector<int>());
    rep(i, M) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    // S->U, T->Uの距離、Xを通過するとき負
    vector<int> StoU(N, INF), TtoU(N, INF);
    StoU[S] = 0, TtoU[T] = 0;

    bfs(S, StoU);
    bfs(T, TtoU);

    // ルートの数え上げ
    ll ans = 0;
    rep(i, N) {
        rep(j, N) {
            
        }
    }
}