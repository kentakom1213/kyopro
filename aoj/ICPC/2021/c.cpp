#include <bits/stdc++.h>
// #include <atcoder/all>
using namespace std;
// using namespace atcoder;
#define rep(i, a, n) for(int i = a; i < n; i++)
#define rrep(i, a, n) for(int i = a; i >= n; i--)
#define inr(l, x, r) (l <= x && x < r)
#define ll long long
#define ld long double

// using mint = modint1000000007;
// using mint = modint998244353;
constexpr int IINF = 1001001001;
constexpr ll INF = 9e18;

template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main(){
    string f;
    cin >> f;

    vector<int> pars(f.size());
    vector<int> st;
    rep(i, 0, f.size()) {
        if (f[i] == '(') {
            st.push_back(i);
        } else if (f[i] == ')') {
            int open = st.front();
            st.pop_back();
            pars[i] = open;
        }
    }

    vector<vector<int>> G(f.size());

    auto build = [&](auto self, int p, int l, int r) {
    };

    build(build, -1, 0, f.size());
    
    return 0;
}
