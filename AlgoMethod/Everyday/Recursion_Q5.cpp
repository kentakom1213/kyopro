//         Q4-5. 部分和問題 (再帰-1)
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/427
// ----------------------------------------

#define _GLIBCXX_DEBUG
#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

ll N, X;
vector<ll> nums;

ll partial_sum(ll i, ll j) {
    // printf("i:%d, j:%d\n", i, j);
    if (i == 0) {
        if (j == 0) return true;
        else return false;
    }
    else {
        bool flag = false;
        if (j >= nums[i-1] && partial_sum(i-1, j-nums[i-1])) flag = true;
        else if (partial_sum(i-1, j)) flag = true;
        return flag;
    }
}

int main() {
    cin >> N >> X;
    nums.assign(N, 0);
    for (int i = 0; i < N; i++) cin >> nums[i];
    cout << (partial_sum(N, X) ? "Yes" : "No") << endl;
}