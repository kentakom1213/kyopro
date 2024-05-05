#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, n) for(int i = a; i < n; i++)
#define rrep(i, a, n) for(int i = a; i >= n; i--)
#define ll long long
#define pii pair<int, int>
#define pll pair<ll, ll>
// constexpr ll MOD = 1000000007;
constexpr ll MOD = 998244353;
constexpr int IINF = 1001001001;
constexpr ll INF = 1LL<<60;

template<class t,class u> void chmax(t&a,u b){if(a<b)a=b;}
template<class t,class u> void chmin(t&a,u b){if(b<a)a=b;}

// 問題
// https://atcoder.jp/contests/DEGwer2023/tasks/DEGwer2023_a

int main(){
    int n, k, t; cin >> n >> k >> t;
    vector<int> a(k);
    vector<int> cnt(n);
    ll ans = 0;
    rep(i, 0, k){
        cin >> a[i];
        a[i]--;
        if(cnt[a[i]] > 0) ans++;
        else cnt[a[i]]++;
    }

    // fix[i] := (修正した回数, 最後に誤植が残っているページ)
    // unfix[i] := 
    vector<pair<ll, ll>> fix(n, {INF, -INF}), unfix(n, {INF, -INF});
    unfix[0] = {cnt[0], -t};
    if(cnt[0] > 0){
        unfix[0] = {0, 0};
        fix[0] = {cnt[0], -t};
    }
    rep(i, 1, n){
        if(cnt[i]){
            if(i-unfix[i-1].second >= t) chmin(unfix[i], (pair<ll, ll>)make_pair(unfix[i-1].first, i));
            if(i-fix[i-1].second >= t) chmin(unfix[i], (pair<ll, ll>)make_pair(fix[i-1].first, i));
            fix[i] = min(fix[i-1], unfix[i-1]);
            fix[i].first++;
        }else{
            //fix[i] = min(fix[i-1], unfix[i-1]);
            unfix[i] = min(unfix[i-1], fix[i-1]);
        }
        //cout << fix[i].first << ' ' << fix[i].second << ' ' << unfix[i].first << ' ' << unfix[i].second << endl;
    }

    cout << ans+min(fix[n-1].first, unfix[n-1].first) << endl;
    
    return 0;
}
