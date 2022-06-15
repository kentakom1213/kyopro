// ハノイの塔

#include <bits/stdc++.h>
using namespace std;
using vstack = vector<deque<int>>;

int N;  // 塔の高さ

void print_stacks(vstack &stacks) {
    for (auto st : stacks) {
        cout << "{ ";
        for (auto val : st) {
            cout << val << " ";
        }
        cout << "}\n";
    }
    cout << "\n\n";
}

void hanoi(int n, vstack &stacks) {
    if (n == 1) {
        from = stacks[0].top();
        stacks[0].pop();
        stacks[2].push_back(from);
    }
    else {
        
    }
}

int main() {
    cin >> N;
    vstack stacks(3);
    for (int i=N; i>=1; i--) {
        stacks[0].push_back(i);
    }

    print_stacks(stacks);
}