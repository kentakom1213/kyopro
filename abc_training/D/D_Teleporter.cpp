//            D - Teleporter
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc167/tasks/abc167_d

// 結構時間かかってしまった

// AC
// ----------------------------------------

// ループを特定 --> modで求める

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll N, K; cin >> N >> K;
    vector<int> A(N);
    for (int i = 0; i < N; i++) {
        int a; cin >> a;
        a--;
        A[i] = a;
    }

    // 地点iに何番目に到達したか
    map<int, int> tour;

    int loop = 0;
    int now = 0;
    for (int i = 0; i < N; i++) {
        if (i == K) {
            cout << now+1 << endl;
            return 0;
        }
        if (tour[now]) {
            loop = i+1 - tour[now];
            break;
        }
        else {
            tour[now] = i+1;  // セット
            now = A[now];
            // printf("%d -> %d\n", now+1, A[now]+1);
        }
    }
    
    // printf("loop-begin:%d, loop-count:%d\n", tour[now]-1, loop);

    K -= tour[now] - 1;
    K %= loop;

    // たどる
    for (int i = 0; i < K; i++) {
        now = A[now];
    }

    cout << now+1 << endl;
}