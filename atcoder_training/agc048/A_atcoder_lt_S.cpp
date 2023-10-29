//             A - atcoder < S             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/agc048/tasks/agc048_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const string atc = "atcoder";

ll test() {
    string S; cin >> S;
    ll ans=0;
    while (S <= atc) {
        bool is_swapped = false;
        rep(i, 0, S.size()-1) {
            if (S[i] < S[i+1]) {
                swap(S[i], S[i+1]);
                ans++;
                is_swapped = true;
                break;
            }
        }
        if (!is_swapped) return -1;
    }
    return ans;
}

int main() {
    int T; cin >> T;
    while (T--) {
        cout << test() << endl;
    }
}