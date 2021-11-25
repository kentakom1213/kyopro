//             D - Wandering
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc182/tasks/abc182_d

// 実装ミス、これが多いと話にならない
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int N; cin >> N;
    vector<ll> A(N);
    rep(i, N) cin >> A[i];

    vector<ll> S(N, A[0]);
    rep(i, N-1) {
        S[i+1] = S[i] + A[i+1];
    }

    vector<ll> triangle(N);  // 三角の頂点を求めていく
    triangle[0] = A[0];
    rep(i, N-1) {
        triangle[i+1] = triangle[i] + S[i+1];
    }

    ll max_triangle_index = max_element(ALL(triangle)) - triangle.begin();
    ll max_S = *max_element(S.begin(), S.begin()+max_triangle_index);

    ll res = 0;
    if (max_triangle_index < N-1) {
        chmax(res, triangle[max_triangle_index]);
        chmax(res, triangle[max_triangle_index] + max_S);
    }
    else {
        chmax(res, triangle[max_triangle_index]);
    }

    cout << res << endl;
}

// 参考
// https://drken1215.hatenablog.com/entry/2020/11/30/031357

// #include <bits/stdc++.h>
// using namespace std;
// typedef long long ll;
// #define rep(i, n) for (int i = 0; i < (int)(n); i++)
// #define ALL(A) A.begin(), A.end()

// int main() {
//     int N; cin >> N;
//     vector<ll> A(N), S(N+1, 0), M(N+1, 0);
//     rep(i, N) {
//         cin >> A[i];
//         S[i+1] = S[i] + A[i];
//         M[i+1] = max(M[i], S[i+1]);  // 累積和Sのiまでの最大値をとっていく
//     }

//     ll res = 0, now = 0;
//     rep(i, N) {
//         res = max(res, now + M[i+1]);
//         now += S[i+1];
//     }
//     cout << res << endl;
// }