#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int t; cin >> t;
    vector<pair<int, string> > res(t);
    rep(i, t) {
        int a, b, c; cin >> a >> b >> c;
        string s; cin >> s;
        res[i] = make_pair(a+b+c, s);
    }
    for (auto [sum, s] : res) {
        cout << sum << " " << s << endl;
    }
}