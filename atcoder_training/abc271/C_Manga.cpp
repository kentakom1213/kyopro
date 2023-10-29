//                C - Manga                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc271/tasks/abc271_c
// ----------------------------------------

// 正規の解法

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N; cin >> N;
    vector<bool> ex(N+2, false);
    vector<int> A(N);
    for (int &a : A) cin >> a;

    int sold = 0;
    for (int i=0; i < N; i++) {
        if (A[i] >= N+2) sold++;
        else if (ex[A[i]]) sold++;
        else ex[A[i]] = true;
    }

    int l=1, r=N+1;
    while (true) {
        while (ex[l]) l++;
        while (r!=0 && !ex[r]) r--;
        if (sold >= 2) {
            sold -= 2;
            ex[l] = true;
        }
        else {
            if (l >= r) break;
            ex[r] = false;
            sold++;
        }
    }
    cout << l-1 << endl;
}
