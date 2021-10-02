//               Q3-1. 友達
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/411

// 多分ここまでしなくてもよかったかな

// AC
// ----------------------------------------

#include <iostream>
#include <vector>
using namespace std;

int main() {
    int N, A, B; cin >> N >> A >> B;
    vector<vector<int>> G(N, vector<int>(N));
    
    for (int i = 0; i < N; i++) {
        string s; cin >> s;
        for (int j = 0; j < N; j++) {
            if (s[j] == 'o') {
                G[i][j] = 1;
                G[j][i] = 1;
            }
        }
    }
    cout << (G[A][B] ? "Yes" : "No") << endl;
}