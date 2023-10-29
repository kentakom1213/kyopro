
#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int main() {
    int N; cin >> N;
    vector<double> A(N);
    for (int i = 0; i < N; i++) cin >> A[i];

    // dpで解く
    double gold = 1, silver = 0;
    vector<int> historyG(N), historyS(N);

    for (int i = 0; i < N; i++) {
        historyG[i] = chmax(gold, silver/A[i]);
        historyS[i] = chmax(silver, gold*A[i]);
        printf("G:%f, S:%f\n", gold, silver);
    }

    // 後ろからたどっていこう
    vector<int> res(N);
    res[N-1] = historyG[N-1];
    bool is_changed = historyG[N-1];
    bool is_G = 1;
    for (int i = N-2; i >= 0; i--) {
        if (is_changed) is_G = !is_G;

        if (is_G) res[i] = is_changed = historyG[i];
        else res[i] = is_changed = historyS[i];
    }

    print_vector(historyG);
    print_vector(historyS);

    // 表示
    for (auto val : res) {
        cout << val << " ";
    }
    cout << endl;
}

// int main() {
//     int N; cin >> N;
//     vector<double> A(N);
//     for (int i = 0; i < N; i++) cin >> A[i];

//     // dpで解く
//     vector<double> gold(N), silver(N);
//     gold[0] = 1, silver[0] = 0;
//     for (int i = 1; i < N; i++) {
//         gold[i] = max(gold[i-1], silver[i-1]/A[i]);
//         silver[i] = max(silver[i-1], gold[i-1]*A[i]);
//     }

//     cout << gold[N-1] << endl;

//     print_vector(gold);
//     print_vector(silver);
// }


// if (is_change_gold && is_change_silver) {
//     for (int j = 0; j < i; j++) swap(historyG[j], historyS[j]);
//     historyG[i] = historyS[i] = 1;
// }
// else if (is_change_gold) {
//     for (int j = 0; j < i; j++) historyG[j] = historyS[j];
//     historyG[i] = 1;
//     historyG[i] = 0;
// }
// else if (is_change_silver) {
//     for (int j = 0; j < i; j++) historyS[j] = historyG[j];
//     historyG[i] = 0;
//     historyG[i] = 1;
// }
// else {
//     historyG[i-1] = historyS[i-1] = 0;
// }