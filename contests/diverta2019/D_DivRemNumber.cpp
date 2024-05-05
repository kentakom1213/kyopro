// D - DivRem Number
// ------------------
// 問題
// https://atcoder.jp/contests/diverta2019/tasks/diverta2019_d
//
// むずい
//
// AC
// ------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<ll>> name(h, vector<ll>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<ll> > Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

template<class T>
void print_set(set<T> S) {
    cerr << "{ ";
    for (auto val : S) {
        cerr << val << ", "[val == *(--S.end())];
    }
    cerr << "}" << endl;
}

// 約数列挙 O(sqrt(N))
set<ll> get_factors(ll n) {
    set<ll> res;
    for (ll i = 1; i*i <= n; i++) {
        if (n % i == 0) {
            res.insert(i);
            res.insert(n/i);
        }
    }
    return res;
}

int main() {
    ll N; cin >> N;
    auto factors = get_factors(N);

    ll res = 0;
    for (auto iter = factors.rbegin(); iter != factors.rend(); iter++) {
        ll v = *iter - 1;
        if (v && N / v == N % v) res += v;
    }
    cout << res << endl;
}
