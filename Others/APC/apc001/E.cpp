
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


int get_ans() {
    int n; cin >> n;
    vector<int> A(n);
    rep(i, n) cin >> A[i];
}

int main() {
    int t; cin >> t;
    vector<int> res(t);
    rep(i, t) {
        res[i] = get_ans();
    }

    rep(i, t) {
        cout << res[i] << endl;
    }
}