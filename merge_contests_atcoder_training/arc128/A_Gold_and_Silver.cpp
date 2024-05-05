//          A - Gold and Silver
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc128/tasks/arc128_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int main() {
    int N; cin >> N;
    vector<ll> A(N);
    rep(i, N) cin >> A[i];

    // dpで最大化
    vector<double> G(N+1), S(N+1);
    G[0] = 1;
    rep(i, N) {
        if (i==0) {
            G[i+1] = G[i];
            S[i+1] = G[i] * A[i];
        } else {
            G[i+1] = max(G[i], S[i] / A[i]);
            S[i+1] = max(S[i], G[i] * A[i]);
        }
    }

    // dp復元
    stack<int> ans;
    double now = G[N];
    int is_gold = 1;
    for (int i=N-1; i>=0; i--) {
        if (now == G[i] || now == S[i]) {
            ans.push(0);
        } else {
            ans.push(1);
            if (is_gold) {
                now *= A[i];
            } else {
                now /= A[i];
            }
            is_gold = !is_gold;
        }
    }

    while (!ans.empty()) {
        cout << ans.top() << " ";
        ans.pop();
    }
    cout << endl;
}
