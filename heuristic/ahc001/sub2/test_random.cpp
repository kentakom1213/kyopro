#include<bits/stdc++.h>
using namespace std;

int randint(int rand_max) {
    return rand() % rand_max;
}

int main() {
    int n; cin >> n;
    for (int i=0; i<100; i++) {
        cout << randint(n) << endl;
    }
}