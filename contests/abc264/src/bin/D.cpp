/*

## 考察
- "atcoder" : 7文字
- 7! = 5040

## 方針
dfsで求められそう
↓
メモ化して再帰？

*/


#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

constexpr int INF = 1 << 30;
constexpr int LEN = 7;  // "atcoder".size()

int main() {
    string S; cin >> S;
    
    // 初期化
    map<string, int> dist;
    string str = "atcoder";
    do {
        dist[str] = INF;
    } while (next_permutation(ALL(str)));
    dist["atcoder"] = 0;

    // ダイクストラ法
    priority_queue<pair<int, string>> pq;
    pq.push({0, "atcoder"});

    while (!pq.empty()) {
        auto [p_cost, prev] = pq.top(); pq.pop();
        if (dist[prev] < p_cost) continue;

        rep(i, 0, LEN) {
            string cur = prev;
            swap(cur[i], cur[i+1]);
            if (dist[cur] > dist[prev] + 1) {
                dist[cur] = dist[prev] + 1;
                pq.push({dist[cur], cur});
            }
        }
    }

    cout << dist[S] << endl;
}
