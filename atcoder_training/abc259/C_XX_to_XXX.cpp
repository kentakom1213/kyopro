//              C - XX to XXX              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc259/tasks/abc259_c
// ----------------------------------------

// ランレングス圧縮

#include <bits/stdc++.h>
using namespace std;

vector<pair<char, int>> RunLengthEncode(string S) {
    vector<pair<char, int>> res;
    char now = S[0];
    int cnt = 1;
    for (int i=1; i < S.size(); i++) {
        if (S[i] == now) cnt++;
        else {
            res.push_back({now, cnt});
            now = S[i];
            cnt = 1;
        }
    }
    res.push_back({now, cnt});
    return res;
}

int main() {
    string S, T; cin >> S >> T;
    auto compS = RunLengthEncode(S);
    auto compT = RunLengthEncode(T);

    bool isOK = compS.size() == compT.size();

    if (isOK) {
        for (int i=0; i < compS.size(); i++) {
            auto [cs, ns] = compS[i];
            auto [ct, nt] = compT[i];
            isOK &= cs == ct;
            isOK &= (ns == 1 && nt == 1) || (ns >= 2 && ns <= nt);
        }
    }

    cout << (isOK ? "Yes" : "No") << endl;
}