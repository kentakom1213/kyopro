
// C - Rotation

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

int main() {
    int N, Q; cin >> N >> Q;
    string S; cin >> S;

    int ptr = 0;
    while (Q--) {
        int t, x; cin >> t >> x;
        if (t == 1) {
            ptr = (N + ptr - x) % N;
        } else {
            cout << S[(N + ptr + x - 1) % N] << "\n";
        }
    }
}

