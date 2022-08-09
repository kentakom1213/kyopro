// https://atcoder.jp/contests/abc076/tasks/abc076_c

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

// 埋め込み可能なら，その座標を返す（一番後ろ）
int can_store(string s, string t) {
    int ls=s.size(), lt=t.size();
    if (ls < lt) return -1;

    int res = -1;
    rep(i, 0, ls) {
        bool isOK = true;
        rep(j, i, lt) {
            if (s[i]!='?' && s[i]!=t[j]) {
                isOK = false;
                break;
            }
        }
        if (isOK) res = i;
    }

    return res;
}

int main() {
    string S, T; cin >> S >> T;

    cout << can_store(S, T) << endl;
}