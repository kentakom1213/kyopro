// 076 - Cake Cut
// ---------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_bx
// ---------------

// 連続した区間であるので尺取り法が使えるが、円形をどう処理すべきか

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
    int n; cin >> n;
    vector<int> A(n);
    rep(i, n) cin >> A[i];
    ll a_tenth = accumulate(ALL(A), 0);

    if (a_tenth % 10) {
        cout << "No" << endl;
        return 0;
    }

    bool isOK = false;
    a_tenth /= 10;

    // 尺取り法
    queue<ll> q;
    ll sum = 0;
    int r = 0;
    rep(l, n) {
        while (sum < a_tenth && r < 2*n && (l < r%n || r%n < l)) {
            printf("++ r:%d, sum:%lld\n", r, sum);
            q.push(A[r%n]);
            sum += A[r%n];
            r += 1;
        }
        printf("%d:%d  sum: %lld\n", l, r, sum);
        if (sum == a_tenth) isOK = true;
        while (sum > a_tenth) {
            printf("--\n");
            ll top = q.front();
            q.pop();
            sum -= top;
        }
    }

    cout << (isOK ? "Yes" : "No") << endl;
}