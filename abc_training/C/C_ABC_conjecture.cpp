//          C - ABC conjecture
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc227/tasks/abc227_c

// AC (解説)
// ----------------------------------------

// O(sqrt(N))なら間に合う
// A,Bの組に関してCを2分探索とかでいけそう

// 解説
// https://atcoder.jp/contests/abc227/editorial/2906

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> point;
typedef vector<vector<int>> Array;

int main() {
    ll N; cin >> N;
    ll cnt = 0;
    for (ll a = 1; a*a*a <= N; a++) {
        for (ll b = a; a*b*b <= N; b++) {
            cnt += floor(N/(a*b))-b+1;
        }
    }
    cout << cnt << endl;
}