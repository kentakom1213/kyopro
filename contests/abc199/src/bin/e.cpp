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
    
    vector<vector<pair<int, int>>> YZ(N + 1);

    rep(i, 0, M) {
        int x, y, z;
        cin >> x >> y >> z;
        YZ[x].push_back({y, z});
    }

    // bitmaskの生成
    vector<int> mask(N + 1);
    rep(i, 0, N) {
        mask[i + 1] = mask[i] << 1 | 1;
    }

    // dp[S] := 数列aを先頭から見たときの集合がSに一致しているものの中で，条件を満たすものの個数
    vector<ll> dp((1 << N), 0);

    dp[0] = 1;

    rep(S, 0, 1 << N) {
        int x = __builtin_popcount(S);

        // 条件を満たすか判定（満たさない場合は0に）
        for (auto [y, z] : YZ[x]) {
            if (__builtin_popcount(S & mask[y]) > z) {
                dp[S] = 0;
            }
        }

        // 次にiを追加する場合
        rep(i, 0, N) if (!((S >> i) & 1)) {
            dp[S | 1 << i] += dp[S];
        }
    }

    // rep(i, 0, 1 << N) {
    //     cout << bitset<18>(i) << ": " << dp[i] << endl;
    // }

    cout << dp[(1 << N) - 1] << endl;

    return 0;
}
