// https://atcoder.jp/contests/indeednow-qualb/tasks/indeednow_2015_qualb_4

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr ll MOD = 1000000007;
constexpr ll mod = 998244353;

template <typename T>
void prll_array(vector<vector<T>>& array) {
    ll H = array.size();

    cerr << "{" << endl;
    for (ll i = 0; i < H; i++) {
        ll W = array.at(i).size();
        cerr << "  {";
        for (ll j = 0; j < W; j++) {
            if (j < W - 1) cerr << array.at(i).at(j) << ", ";
            else cerr << array.at(i).at(j);
        }
        cerr << "}," << endl;
    }
    cerr << "}" << endl;
}

int main() {
    ll N, C; cin >> N >> C;
    vector<ll> A(N);
    rep(i, 0, N) cin >> A[i];

    // 色ごとに出てくるidxをベクタにまとめる
    vector<vector<ll>> colors(C, {-1});
    rep(i, 0, N) {
        colors[A[i]-1].push_back(i);
    }
    rep(i, 0, C) colors[i].push_back(N);

    // 色ごとに数える
    for (auto col : colors) {
        ll ans = N * (N+1) / 2;
        rep(i, 0, col.size()-1) {
            ll span = col[i+1] - col[i] - 1;
            ans -= span * (span+1) / 2;
        }
        cout << ans << endl;
    }
}
