
// #include <bits/stdc++.h>
// using namespace std;

// int main() {
//     int N; cin >> N;

//     int span = 1000000;  // 間に合わない
//     vector<int> login(span, 0);
//     for (int i = 0; i < N; i++) {
//         int a, b; cin >> a >> b;
//         login[a]++;
//         login[a+b]--;
//     }

//     vector<int> res(N+1, 0);
//     int now = 0;
//     for (int i = 0; i < span; i++) {
//         now += login[i];
//         res[now]++;
//     }

//     for (int i = 1; i <= N; i++) {
//         cout << res[i] << endl;
//     }
// }


// 座圧？？
#include <bits/stdc++.h>
using namespace std;

int main() {
    int N; cin >> N;

    vector<int> user(N+1, 0);
    

    for (int i = 0; i < N; i++) {
        int a, b; cin >> a >> b;
        login[a]++;
        login[a+b]--;
    }

}