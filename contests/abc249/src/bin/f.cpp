#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, n) for(int i = a; i < n; i++)
#define rrep(i, a, n) for(int i = a; i >= n; i--)
#define inr(l, x, r) (l <= x && x < r)
#define ll long long
#define ld long double

using pil = pair<int, ll>;
constexpr ll INF = 9e18;

int main(){
    int N, K;
    cin >> N >> K;
    vector<pil> TY(N);

    for (auto &[t, y] : TY) {
        cin >> t >> y;
    }

    // 後ろから見たとき，大きい順に並べた配列
    priority_queue<ll> pq;

    

    return 0;
}
