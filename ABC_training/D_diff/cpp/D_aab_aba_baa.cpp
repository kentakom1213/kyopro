//             D - aab aba baa 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc202/tasks/abc202_d

// 参考
// https://blog.hamayanhamayan.com/entry/2021/05/22/225424

// AC
// ----------------------------------------

// aがp個，bがq個ある時にaから始まる文字列の個数は，
// (a+b-1)! / ((a-1)!b!) = combi(a+b-1, b)

// Kに関して最初の文字がaかbかを同定するcombiの値を求め，それをKから引いていく操作を繰り返せば良い

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;


// 二項係数を求める
ll combi(int a, int b) {
    if (a == 0 || b == 0) return 1;
    else {
        return (a - b + 1) * combi(a, b-1) / b;
    }
}

int main() {
    int a, b; cin >> a >> b;
    ll K; cin >> K;

    string res = "";
    while (a && b) {
        ll from_a = combi(a+b-1, b);

        if (from_a < K) {
            res.push_back('b');
            K -= from_a;
            b--;
        }
        else {
            res.push_back('a');
            a--;
        }
    }
    cout << res << string(a, 'a') << string(b, 'b') << endl;
}