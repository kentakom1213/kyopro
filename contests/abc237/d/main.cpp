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
    ll N; cin >> N;
    string S; cin >> S;
    list<ll> linked_list;

    linked_list.push_back(0);

    auto iter = linked_list.begin();
    rep(i, N) {
        ll val = i+1;
        if (S[i] == 'L') {
            iter = linked_list.insert(iter, val);
        } else {
            iter = linked_list.insert(++iter, val);
        }
    }

    for (auto v : linked_list) {
        cout << v << " ";
    }
    cout << endl;
}