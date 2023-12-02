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
// https://atcoder.jp/contests/DEGwer2023/tasks/j


int main(){
    vector<vector<int>> dp(100005);
    dp[1] = {1};
    int l = 2;
    rep(i, 1, 100005){
        if(dp[i].size() == 0) continue;
        int start = dp[i].back();
        
        rep(j, max(l-i, start+1), i+2){
            if(i+j >= 100005) continue;
            if(dp[i+j].size()) continue;
            dp[i+j] = dp[i];
            dp[i+j].push_back(j);
            l = i+j;
        }
    }

    int t; cin >> t;
    vector<int> x_1 = {2, 3, 4};
    while(x_1.back()*2 <= 100000){
        x_1.push_back(x_1.back()*2);
    }
    while(t--){
        int x, m; cin >> x >> m;
        if(x == 1){
            cout << x_1.size() << endl;
            for(auto dd: x_1){
                cout << dd << ' ';
            }
            cout << endl;
        }else if(x == m){
            // 1~m-1 を作れるかどうか
            if(1 <= dp[x-1].size() && dp[x-1].size() <= 20){
                cout << dp[x-1].size() << endl;
                for(auto dd: dp[x-1]){
                    cout << dd << ' ';
                }
                cout << endl;
            }
        }else{
            // x < m
            if(dp[x-1].size() == 0){
                cout << -1 << endl;
                continue;
            }

            vector<int> ans = dp[x-1];
            if(x+1 <= m) ans.push_back(x+1);
            int r = 2*x+1;
            int sum = 2*x;
            while(r <= m && ans.size() <= 20){
                ans.push_back(r);
                sum += r;
                r = sum-x;
            }
            
            if(r >= m && ans.size() <= 20){
                cout << ans.size() << endl;
                for(auto aa: ans){
                    cout << aa << ' ';
                }
                cout << endl;
            }else{
                cout << -1 << endl;
            }
        }

    }
    
    return 0;
}
