//             C - おいしいたこ焼きの売り方            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc005/tasks/abc005_3
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int T, N, M;
const int MAX_SIZE = 200;
int A[MAX_SIZE], B[MAX_SIZE];

int main() {
    cin >> T >> N;
    rep(i, N) cin >> A[i];
    cin >> M;
    rep(i, M) cin >> B[i];

    bool isOK = true;
    int tako = 0;
    rep(i, M) {
        int kyaku = B[i];
        while (tako < MAX_SIZE && A[tako] + T < kyaku) tako++;
        if (tako >= N || A[tako] > kyaku) {
            isOK = false;
            break;
        } else {
            tako++;
        }
    }

    cout << (isOK ? "yes" : "no") << endl;
}
