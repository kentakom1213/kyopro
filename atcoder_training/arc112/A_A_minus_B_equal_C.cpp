//            A - "A - B = C"
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc112/tasks/arc112_a

// 一生解けん
// ----------------------------------------

// 1つのクエリについて O(sqrt(N)) 程度で求める必要がある

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

ll cntABC(int L, int R) {
    ll X = R - 2*L + 1;
    if (X <= 0) return 0;
    if (X & 1) {
        X = (X+1) / 2;
        return 2*X*X - X;
    }
    X /= 2;
    return 2*X*(X-1)-X;
}

int main() {
    int T; cin >> T;
    while (T--) {
        int l, r; cin >> l >> r;
        cout << cntABC(l, r) << endl;
    }
}
