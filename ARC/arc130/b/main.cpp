
// クエリはたかだか 3*10^5 である

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()

int main() {
    ll H, W, C, Q;
    cin >> H >> W >> C >> Q;
    vector<ll> qT(Q), qN(Q), qC(Q), colors(C, 0);
    for (int i = Q-1; i >= 0; i--) {
        ll t, n, c;
        cin >> t >> n >> c;
        n--, c--;
        qT[i] = t;
        qN[i] = n;
        qC[i] = c;
    }

    ll r_cnt = 0, c_cnt = 0;
    map<ll, ll> row, colmn;
    rep(i, Q) {
        ll t = qT[i], n = qN[i], c = qC[i];

        // print_vector(colors);
        // cerr << t << " " << n << " " << c << endl;

        // rowについて処理
        if (t == 1) {
            if (row.find(n) == row.end()) {
                colors[c] += W - c_cnt;
                row[n] = W-1;
                r_cnt++;
            }
            else {
                continue;
                // cerr << row[n] << " " << c_cnt << endl;
                // colors[c] += row[n] - c_cnt;
            }
            row[n]--;
        }
        // colmnについて処理
        else {
            if (colmn.find(n) == colmn.end()) {
                colors[c] += H - r_cnt;
                colmn[n] = H-1;
                c_cnt++;
            }
            else {
                continue;
                // colors[c] += colmn[n] - r_cnt;
            }
            colmn[n]--;
        }
    }

    rep(i, C) {
        cout << colors[i] << " ";
    }
    cout << endl;
}