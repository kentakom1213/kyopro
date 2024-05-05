//            D - Snuke Prime
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc188/tasks/abc188_d
// ----------------------------------------

/* comment
## 方針
- 座標圧縮 + いもす法

めっちゃバグった
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

template<class T>
T index(const vector<T> &arr, T x) {
    auto ptr = lower_bound(arr.begin(), arr.end(), x);
    return ptr - arr.begin();
}

int main() {
    ll N, C; cin >> N >> C;
    vector<array<ll, 3>> seg(N);
    vector<ll> comp;

    for (auto &[a, b, c] : seg) {
        cin >> a >> b >> c;
        a--;
        comp.push_back(a);
        comp.push_back(b);
    }

    // 座標圧縮
    sort(ALL(comp));

    // いもす法
    vector<ll> imos(2*N+10, 0);
    for (auto [a, b, c] : seg) {
        imos[index(comp, a)] += c;
        imos[index(comp, b)] -= c;
    }

    // print_vector(comp);
    // print_vector(imos);

    // 累積和をとる
    ll ans = 0;
    ll s = 0;
    rep(i, comp.size()-1) {
        s += imos[i];
        ll a = comp[i], b = comp[i+1];
        ans += min(s, C) * max(b-a, 0ll);
        // printf("(%lld, %lld) * min(%lld, %lld) -> %lld\n", a, b, s, C, ans);
    }

    cout << ans << endl;
}
