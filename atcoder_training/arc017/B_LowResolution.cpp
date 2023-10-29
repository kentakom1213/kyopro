//            B - 解像度が低い。
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc017/tasks/arc017_2
// ----------------------------------------

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

int main() {
    ll N, K; cin >> N >> K;
    vector<ll> A(N);
    rep(i, N) cin >> A[i];

    vector<ll> inc;
    // 尺取り法で連続する増加列を探索
    queue<ll> q;
    ll r = 0;
    for (ll l : A) {
        while (q.empty() || q.back() < A[r]) {
            q.push(A[r]);
            r++;
        }

        inc.push_back(q.size());
        if (r == N) break;

        while (!q.empty()) {
            cerr << q.front() << " ";
            q.pop();
        }
        cerr << endl;
    }

    ll res = 0;
    for (auto i : inc) {
        res += max(0LL, i - K + 1);
    }
    cout << res <<  endl;
}
