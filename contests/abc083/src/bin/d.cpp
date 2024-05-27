#include <bits/stdc++.h>
// #include <atcoder/all>
using namespace std;
// using namespace atcoder;
#define rep(i, a, n) for(int i = a; i < n; i++)
#define rrep(i, a, n) for(int i = a; i >= n; i--)
#define inr(l, x, r) (l <= x && x < r)
#define ll long long
#define ld long double

// using mint = modint1000000007;
// using mint = modint998244353;
constexpr int IINF = 1001001001;
constexpr ll INF = 9e18;

template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

string S;

// 長さk以上の区間で可能かを判定
bool can(int k) {
    vector<int> acc(S.size() + 1);
    int val = 0;

    // imos法
    rep(i, 0, S.size() - k + 1) {
        val += acc[i];
        // 変更が必要な場合，iからi+kまでを反転
        if (val % 2 != (S[i] - '0')) {
            val += 1;
            acc[i] += 1;
            acc[i + k] -= 1;
        }
    }

    // 累積和
    rep(i, 0, S.size()) {
        acc[i + 1] += acc[i];
        acc[i + 1] %= 2;
    }

    // 先頭からk個が0であればok
    rep(i, 0, k) {
        if (acc[i] ^ (S[i] - '0') != 0) {
            return false;
        }
    }

    return true;
}

int main(){
    cin >> S;

    int ok = 1, ng = S.size() + 1;

    while (ng - ok > 1) {
        int mid = (ok + ng) / 2;

        if (can(mid)) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    cout << ok << endl;
    
    return 0;
}
