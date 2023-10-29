//         D - Multiple of 2019
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc164/tasks/abc164_d
// ----------------------------------------

// S[i:j] -> iを前列挙し、jを高速に数え上げる → x
// けんちょんさんすごい

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<long long>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

// modをカウント
int cnt[2020];

int main() {
    string S; cin >> S;
    int N = S.size();

    ll cur = 0, mul = 1;
    cnt[cur]++;
    rep(i, 0, N) {
        ll d = S[N-i-1] - '0';
        cur = (cur + d * mul) % 2019;
        mul = (mul * 10) % 2019;
        cnt[cur]++;
    }

    ll ans = 0;
    rep(i, 0, 2019) {
        ans += cnt[i] * (cnt[i] - 1) / 2;
    }

    cout << ans << endl;
}
