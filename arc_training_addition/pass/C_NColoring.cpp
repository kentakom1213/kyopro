//             C - ℕ Coloring
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc115/tasks/arc115_c
// ----------------------------------------

// 素数の時は変えなくてもいい!
// エラトステネスの篩を使う？
// O(nloglogn)


#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> point;
typedef vector<vector<int>> Array;

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
    ll N; cin >> N;
    // 篩
    vector<int> sieve(100000, 1);
    sieve[0] = sieve[1] = 0;
    for (int i = 2; i < N+10; i++) {
        for (int j = 2; i*j < N+1; j++) {
            if (sieve[i]) sieve[i*j] = 0;
        }
    }

    sieve[2] = 0;
    int cnt = 1;
    for (int i = 2; i < N+2; i++) {
        cout << cnt << " ";
        if (sieve[i] == 0) cnt++;
    }
    cout << endl;
}

// 方針ミス