//          055 - Select 5（★2） 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_bc

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N, P, Q; cin >> N >> P >> Q;
    vector<int> A(N);
    for (int i = 0; i < N; i++) cin >> A[i];

    int res = 0;
    for (int a = 0; a < N; a++) {
        for (int b = a+1; b < N; b++) {
            for (int c = b+1; c < N; c++) {
                for (int d = c+1; d < N; d++) {
                    for (int e = d+1; e < N; e++) {
                        ll mod = A[a] % P;
                        mod = (mod * A[b]) % P;
                        mod = (mod * A[c]) % P;
                        mod = (mod * A[d]) % P;
                        mod = (mod * A[e]) % P;
                        
                        if (mod == Q) res++;
                    }
                }
            }
        }
    }

    cout << res << endl;
}