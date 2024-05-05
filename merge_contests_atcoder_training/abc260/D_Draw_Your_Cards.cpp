//           D - Draw Your Cards           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc260/tasks/abc260_d
// ----------------------------------------

// 実装つまった

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
constexpr ll INF = numeric_limits<long long>::max();

template <typename T>
void print_array(vector<vector<T>>& array) {
    int H = array.size();

    cerr << "{" << endl;
    for (int i = 0; i < H; i++) {
        int W = array.at(i).size();
        cerr << "  {";
        for (int j = 0; j < W; j++) {
            if (j < W - 1) cerr << array.at(i).at(j) << ", ";
            else cerr << array.at(i).at(j);
        }
        cerr << "}," << endl;
    }
    cerr << "}" << endl;
}

template<class T, class U>
void print_map(map<T, U> dict) {
    cerr << "{ ";
    for (auto &[a, b] : dict) {
        cerr << a << ":" << b << ", ";
    }
    cerr << "}" << endl;
}

int main() {
    ll N, K; cin >> N >> K;

    // stackのtopと要素をmapで管理
    map<ll, ll> top_elem;
    vector<vector<ll>> stacks;
    vector<int> ans(N, -1);

    rep(i, 1, N+1) {
        ll p; cin >> p;

        // top_elem の要素のうち、pより大きい最小のものを求める
        auto le_min = top_elem.lower_bound(p);

        if (le_min == top_elem.end()) {
            stacks.push_back({p});
            top_elem[p] = stacks.size()-1;
        } else {
            ll st = le_min->second;
            stacks[st].emplace_back(p);
            top_elem[p] = st;
            top_elem.erase(le_min);
        }

        // カードの枚数がK枚に到達したとき
        ll st = top_elem[p];
        if (stacks[st].size() == K) {
            for (auto v : stacks[st]) {
                ans[v-1] = i;
            }
            // 要素を削除
            top_elem.erase(p);
        }
    }

    rep(i, 0, N) {
        cout << ans[i] << endl;
    }
}
