//              Kaprekar Number
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc192/tasks/abc192_c

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int N, K;
    cin >> N >> K;

    rep (i, K) {
        string g1, g2;
        g1 = g2 = to_string(N);
        sort(g1.begin(), g1.end(), greater<int>());
        sort(g2.begin(), g2.end());

        N = stoi(g1) - stoi(g2); 
    }
    cout << N << endl;    
}