//         046 - I Love 46（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_at
// ----------------------------------------

// 全探索するとO(n^3)で間に合わない
// あらかじめmodをとっておく
// 46^3 でできる
// 計算量はO(n)

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
constexpr int MOD = 46;

template<class T>
void print_set(set<T> S) {
    cerr << "{ ";
    for (auto val : S) {
        cerr << val << ", "[val == *(--S.end())];
    }
    cerr << "}" << endl;
}


int main() {
    int N; cin >> N;
    map<ll, ll> A, B, C;
    rep(i, 3*N) {
        ll a; cin >> a;
        switch (i / N) {
            break; case 0: A[a % MOD]++;
            break; case 1: B[a % MOD]++;
            break; case 2: C[a % MOD]++;
        }
    }

    ll res = 0;
    for (auto [a, i] : A) {
        for (auto [b, j] : B) {
            for (auto [c, k] : C) {
                if ((a+b+c) % MOD == 0) {
                    res += i*j*k;
                }
            }
        }
    }

    cout << res << endl;
}