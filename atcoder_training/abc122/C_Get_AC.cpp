//              C - Get AC
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc122/tasks/abc122_c

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> point;
typedef vector<vector<int>> Array;

int main() {
    int N, Q; cin >> N >> Q;
    string S; cin >> S;
    vector<int> AC(N);  // 文字A,Cの累積和
    bool is_prevA = (S[0] == 'A');
    for (int i = 0; i <= N; i++) {
        if (S[i] == 'A') {
            AC[i] = AC[i-1];
            is_prevA = true;
        }
        else if (S[i] == 'C' && is_prevA) {
            AC[i] = AC[i-1] + 1;
            is_prevA = false;
        }
        else {
            AC[i] = AC[i-1];
            is_prevA = false;
        }
    }

    // print_vector(AC);

    rep(i, Q) {
        int l, r; cin >> l >> r;
        l--, r--;
        // 区間に存在するA, Cの個数を計算
        int cnt = AC[r] - AC[l];
        cout << cnt << endl;
    }
}