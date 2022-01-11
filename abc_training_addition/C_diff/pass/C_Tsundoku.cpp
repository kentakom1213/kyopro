//              C - Tsundoku
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc172/tasks/abc172_c

// これむずい
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

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int main() {
    ll n, m, k; cin >> n >> m >> k;
    vector<ll> books(n+m);
    rep(i, n) cin >> books[n-i-1];
    rep(i, m) cin >> books[n+i];

    queue<ll> inchworm;
    ll sum = 0, now_num = 0, max_num = 0;
    for (int i = 0; i < n+m; i++) {
        ll t = books[i];

        while (sum + t > k && now_num) {
            ll left = inchworm.front();
            inchworm.pop();
            sum -= left;
            now_num--;
        }
        // printf("l:%lld, r:%d\n", i-now_num, i);

        inchworm.push(t);
        sum += t;
        now_num++;

        // 範囲に0が含まれているか
        if (i-now_num < n && n <= i) {
            chmax(max_num, now_num);
        }
    }

    cout << max_num << endl;
}