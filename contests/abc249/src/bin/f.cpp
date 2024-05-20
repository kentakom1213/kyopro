#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, n) for (int i = a; i < n; i++)
#define rrep(i, a, n) for (int i = a; i >= n; i--)
#define inr(l, x, r) (l <= x && x < r)
#define ll long long
#define ld long double

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

using pil = pair<int, ll>;
constexpr ll INF = 9e18;

int main() {
    int N, K;
    cin >> N >> K;
    vector<pil> TY = {{1, 0}};

    rep(i, 0, N) {
        int t;
        ll y;
        cin >> t >> y;
        TY.push_back({t, y});
    }

    ll ans = -INF;

    // 後ろから見たとき，採用した要素を小さい順に並べた配列
    priority_queue<ll, vector<ll>, greater<ll>> T;

    // 後ろから見たとき，無視した要素を大きい順に並べた配列
    // - Sの要素は0未満
    priority_queue<ll> S;

    // 後ろから見たときの総和
    ll sum_t = 0;

    // クエリ1の個数
    int cnt1 = 0;

    // 後ろから見ていく
    rrep(i, N, 0) {
        auto [t, y] = TY[i];

        if (t == 1) {
            // (K - cnt1)個以下消せる
            if (S.size() > K - cnt1) {
                ll s_min = S.top();
                S.pop();
                T.push(s_min);
                sum_t += s_min;
            }

            ll res = y + sum_t;
            chmax(ans, res);

            // これ以上無視できない場合，終了
            if (++cnt1 > K) {
                break;
            }
        } else {
            if (y >= 0) {
                T.push(y);
                sum_t += y;
            } else {
                S.push(y);
                // 無視できる数を超えた場合
                if (S.size() > K - cnt1) {
                    ll s_min = S.top();
                    S.pop();
                    T.push(s_min);
                    sum_t += s_min;
                }
            }
        }
    }

    cout << ans << endl;

    return 0;
}
