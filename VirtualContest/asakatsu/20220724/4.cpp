// https://atcoder.jp/contests/abc125/tasks/abc125_d

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <class T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <class T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    ll N; cin >> N;
    vector<ll> A(N);
    rep(i, 0, N) cin >> A[i];

    // 全ての文字を正にできるか
    bool can_pos_all = true;
    rep(i, 0, N) can_pos_all ^= A[i] < 0;

    // 総和を求めつつ，絶対値の最小値を求める
    ll sum=0, mini=(ll)1e20;
    rep(i, 0, N) {
        sum += abs(A[i]);
        chmin(mini, abs(A[i]));
    }

    // 全ての文字を正にできない場合，絶対値が最小のものを負にする
    if (!can_pos_all) sum -= mini * 2;

    cout << sum << endl;
}