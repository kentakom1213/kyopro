
// 半分全列挙

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


int main() {
    int N; cin >> N;
    ll X; cin >> X;
    vector<ll> A(N);
    rep(i, N) cin >> A[i];

    // 商をとっていく
    vector<int> diff(N, 1);
    rep(i, N-1) {
        diff[i+1] = A[i+1] / A[i];
    }

    vector<ll> res(N, 0);  // 何が何枚あればいいか

    // 貪欲に取る
    int now_coin = N-1;
    while (X) {
        if (A[now_coin] <= X) {
            X -= A[now_coin];
            res[now_coin]++;
        }
        else now_coin--;
    }

    // x = res[i] について
    // A[i] * x == A[i-1] * 1 - A[i] * abs(x - diff[i])
    // x > abs(x - diff[i])  ならば交換
    for (int i = N-1; i >= 1; i--) {
        int x = res[i-1];
        if (x) {
            if (x > abs(x - diff[i]) + 1) {
                res[i]++;
                res[i-1] = x - diff[i];
            }
        }
    }

    for (auto& v : res) v = abs(v);
    cout << accumulate(ALL(res), 0) << endl;
}
