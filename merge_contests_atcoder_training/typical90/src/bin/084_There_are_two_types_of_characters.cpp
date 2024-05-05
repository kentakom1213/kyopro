// 084 - There are two types of characters（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_cf
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<long long>> Array;
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
    ll N; cin >> N;
    string S; cin >> S;

    // ランレングス圧縮
    vector<ll> comp;
    ll cnt = 0;
    char prev = S[0];
    for (char c : S) {
        if (c == prev) cnt++;
        else {
            comp.push_back(cnt);
            cnt = 1;
        }
        prev = c;
    }
    if (cnt) comp.push_back(cnt);

    // 余事象を計算
    ll ans = N * (N-1) / 2;
    for (ll n : comp) {
        ans -= n * (n-1) / 2;
    }
    cout << ans << endl;
}
