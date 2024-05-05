// D - Querying Multiset
// ----------------------
// 問題
// https://atcoder.jp/contests/abc212/tasks/abc212_d
//
// pop済みの要素を消すのが大事
//
// AC
// ----------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int> > Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

// template<class T, class U>
// void print_map(map<T, U> dict) {
//     cerr << "{\n";
//     for (auto &[a, b] : dict) {
//         cerr << "   {" << a << ", " << b << "},\n";
//     }
//     cerr << "}" << endl;
// }

int main() {
    int Q; cin >> Q;
    vector<vec2> queries(Q);
    for (auto& [q, x] : queries) {
        int a; cin >> a;
        if (a == 3) q = 3;
        else {
            int b; cin >> b;
            q = a;
            x = b;
        }
    }
    
    map<ll, ll> balls;  // ボールの数字とその個数
    ll added = 0;       // Q2で追加された数
   
    rep(i, Q) {
        int q = queries[i].first;
        ll x = queries[i].second;
        switch (q) {
            case 1:
                balls[x - added]++;
                break;
            case 2:
                added += x;
                break;
            case 3:
                auto min_val = balls.begin();
                while (!(min_val->second)) {
                    min_val = balls.erase(min_val);
                }
                cout << (min_val->first) + added << endl;
                balls[min_val->first]--;
                break;
        }

        // print_map(balls);
    }
}
