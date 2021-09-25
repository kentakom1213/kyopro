//             Q1-3. コイン問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/360

// AC
// ----------------------------------------

#include <iostream>
#include <vector>
using namespace std;

const vector<int> COINS = {50, 10, 5, 1};

int main() {
    int X; cin >> X;
    vector<int> coin_num(4);
    for (int i = 0; i < 4; i++) cin >> coin_num[i];

    int res = 0;
    for (int i = 0; i < 4; i++) {
        while (X >= COINS[i] && coin_num[i]) {
            X -= COINS[i];
            coin_num[i]--;
            res++;
        }
    }
    cout << res << endl;
}