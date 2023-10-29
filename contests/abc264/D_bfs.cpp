
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

constexpr int LEN = 6;  // len("atcoder")
set<string> mem;

int dist(string S) {
    queue<pair<string, int>> q;
    q.push({"atcoder", 0}); mem.insert("atcoder");
    while (!q.empty()) {
        auto [cur, cost] = q.front(); q.pop();
        if (cur == S) return cost;
        rep(i, 0, LEN) {
            string nxt = cur;
            swap(nxt[i], nxt[i+1]);
            if (mem.find(nxt) != mem.end()) continue;
            q.push({nxt, cost+1});
            mem.insert(nxt);
        }
    }
}

int main() {
    string S; cin >> S;
    cout << dist(S) << endl;
}