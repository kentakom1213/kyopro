//                C - To 3
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc182/tasks/abc182_c

// AC
// ----------------------------------------

// 桁ごとに消す・消さないを考えると 2^18 = 262144 間に合う

#include <iostream>
#include <vector>
using namespace std;

int main() {
    string N; cin >> N;

    int max_len = -1;
    for (int i = 0; i < (1 << N.size()); i++) {
        int sum = 0, len = 0;
        for (int j = 0; j < N.size(); j++) {
            if ((i >> j) & 1) {
                sum += (N[j] - '0');
                len++;
            }
        }
        if (sum > 0 && sum % 3 == 0) max_len = max(max_len, len);
    }
    if (max_len == -1) cout << max_len << endl;
    else cout << N.size() - max_len << endl;
}