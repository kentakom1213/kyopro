#include <bits/stdc++.h>
// #include <atcoder/all>
using namespace std;
// using namespace atcoder;
#define rep(i, a, n) for (int i = (a); i < (n); i++)
#define rrep(i, a, n) for (int i = (a); i >= (n); i--)
#define inr(l, x, r) ((l) <= (x) && (x) < (r))
#define ll long long
#define ld long double

// using mint = modint1000000007;
// using mint = modint998244353;
constexpr int IINF = 1001001001;
constexpr ll INF = 9e18;

template <typename T>
inline bool chmax(T &a, const T b) {
    if (a < b) {
        a = b;
        return true;
    }
    return false;
}
template <typename T>
inline bool chmin(T &a, const T b) {
    if (a > b) {
        a = b;
        return true;
    }
    return false;
}

int N, M;

int main() {
    cin >> N >> M;
    vector<array<int, 3>> XYZ(M, {0, 0, 0});
    for (auto &[x, y, z] : XYZ) {
        cin >> x >> y >> z;
        // 0-indexedに変換
        y--;
    }

    // dp[S] := 数列aを先頭から見たときの集合がSに一致しているものの中で，条件を満たすものの個数
    vector<ll> dp((1 << N), 0);

    dp[0] = 1;

    rep(S, 0, 1 << N) {
        // 次にiを追加する場合
        rep(i, 0, N) if (!((S >> i) & 1)) {

            // iを追加
            int T = S | (1 << i);

            // すべての条件を確認
            vector<int> cnt(N + 1, 0);

            rep(j, 0, N) {
                cnt[j + 1] = cnt[j] + ((T >> j) & 1);
            }

            // cout << bitset<18>(T) << ": ";
            // rep(j, 0, N + 1) {
            //     cout << cnt[j] << " ";
            // }
            // cout << endl;

            // すべての条件を満たしているか確認
            bool isok = true;

            for (auto [x, y, z] : XYZ) {
                isok &= (__builtin_popcount(T) != x) || (cnt[y + 1] <= z);
            }

            if (isok) {
                dp[T] += dp[S];
            }
        }
    }

    // rep(i, 0, 1 << N) {
    //     cout << bitset<18>(i) << ": " << dp[i] << endl;
    // }

    cout << dp[(1 << N) - 1] << endl;

    return 0;
}
