
// P, Qは(1..N)の順列であることを利用したい
// 倍数の列挙 -> LIS

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
constexpr ll INF = 1LL << 40;

int main() {
    ll N; cin >> N;
    vector<ll> P(N), Q(N);
    rep(i, N) cin >> P[i];
    rep(i, N) cin >> Q[i];

    // Qの値がどこにあるかを表す配列
    // index[val-1] = (Q[i] = valとなるインデックス)
    vector<ll> index(N+1);
    rep(i, N) index[Q[i]] = i ;

    // 約数を列挙
    vector<set<ll>> mul_item(N);
    rep(i, N) {
        ll p = P[i];
        for (ll j = 1; p*j <= N; j++) {
            ll ind = index[p*j];
            mul_item[i].insert(ind);
        }
    }

    // LISで部分列の長さを求める
    
}