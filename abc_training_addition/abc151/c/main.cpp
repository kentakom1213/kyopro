#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()


int main() {
    int N, M; cin >> N >> M;
    map<int, pair<int, int>> result;

    rep(i, M) {
        int p; cin >> p;
        string S; cin >> S;
        // printf("%d: %s\n",p, S.c_str());
        if (S == "AC") {
            result[p].first = 1;
        }
        if (S == "WA" && !result[p].first) {
            result[p].second++;
        }
    }

    int ac = 0, pena = 0;
    for (auto itr=result.begin(); itr != result.end(); itr++) {
        auto q = itr->second;
        // printf("{%d, %d}\n",q.first, q.second);
        if (q.first) {
            ac++;
            pena += q.second;
        }
    }

    cout << ac << " " << pena << endl;
}