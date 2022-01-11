//               C - Repsept
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc174/tasks/abc174_c

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

ll ans = 0;

ll add_seven(ll n) {
    string res = to_string(n) + '7';
    ans++;
    return stoll(res);
}

int main() {
    string n_str; cin >> n_str;
    int len = n_str.length(); // 桁数
    ll N = stoll(n_str); // N

    // 割り算の筆算を実装しよう
    string str_sevens(len, '7');
    ll sevens = stoll(str_sevens);
    ans = len;

    set<ll> mods;
    while (sevens != 0) {
        if (sevens < N) sevens = add_seven(sevens);
        sevens = sevens % N;

        if (mods.find(sevens) != mods.end()) break;
        else mods.insert(sevens);
    }

    if (sevens == 0) cout << ans << endl;
    else cout << (-1) << endl;

}