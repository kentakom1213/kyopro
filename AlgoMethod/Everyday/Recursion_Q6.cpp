//         Q4-6. 部分和問題 (再帰-2)
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/438
// ----------------------------------------

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

int n, x;
vector<int> A;

// Aのi個目までのの要素で、jを作ることができるか
int rec(int i, int j) {
    if (i == 0) {
        if (j == 0)
    }
}

int main() {
    cin >> n >> x;
    A.assign(n, 0);
    rep(i, n) cin >> A[i];

        
}
