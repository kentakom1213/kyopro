//            C - Adjacent Swaps           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc250/tasks/abc250_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N, Q; cin >> N >> Q;
    vector<int> X(Q);
    rep(i, 0, Q) cin >> X[i];

    vector<int> arr(N);
    map<int, int> idx;
    rep(i, 0, N) {
        arr[i] = i + 1;
        idx[i+1] = i;
    }

    // クエリ処理
    for (int x : X) {
        int &left = arr[idx[x]];
        int &right = arr[idx[x] + (idx[x]==N-1 ? -1 : 1)];

        // swap index (XOR swap)
        idx[left] ^= idx[right];
        idx[right] ^= idx[left];
        idx[left] ^= idx[right];

        // swap array
        swap(left, right);
    }

    rep(i, 0, N) cout << arr[i] << " ";
    cout << endl;
}
