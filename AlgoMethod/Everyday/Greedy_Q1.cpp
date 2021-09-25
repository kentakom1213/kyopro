//           Q1-1. 1 円玉と 5 円玉
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/359

// AC
// ----------------------------------------

#include <iostream>
#include <vector>
using namespace std;

const vector<int> COINS = {1, 5};

int main() {
    int N; cin >> N;

    int res = 0;
    for (int i = 0; i < COINS.size(); i++) {
        int coin = COINS[COINS.size() - i - 1];
        while (N >= coin) {
            N -= coin;
            res++;
        }
    }
    cout << res << endl;
}