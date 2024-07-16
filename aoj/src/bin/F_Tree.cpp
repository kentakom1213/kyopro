// F - Tree
// ----------
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2667
// ----------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int> > Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int n, q; cin >> n >> q;
    vector<int> A(n), B(n), queries(q);
    rep(i, n) cin >> A[i] >> B[i];
    rep(i, q) cin >> queries[i];


}
