//           A - Smaller XOR
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc129/tasks/arc129_a
// ----------------------------------------

// 解説
// (x >> i)&1 == 1 となる最大のiについて
//   (N >> i)&1 == 1 のとき -> (N xor x) > N
//   (N >> i)&1 == 0 のとき -> (N xor x) < N

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

int main() {
    ll N, L, R; cin >> N >> L >> R;
    int MAX = 0;
    while (N >> MAX) MAX++;
    MAX++;

    // Nを2進法で表したときのi桁目が1であるとき
    ll cnt = 0;
    for (int i = 0; i <= MAX; i++) if ((N>>i)&1) {
        ll max_bit = (1LL << (i+1)) - 1;
        ll min_bit = 1LL << i;
        ll upper = min(R, max_bit);
        ll lower = max(L, min_bit);
        if (lower <= upper) cnt += upper - lower + 1;
    }

    cout << cnt << endl;
}
