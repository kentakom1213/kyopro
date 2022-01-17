//         EX. エラトステネスの区間篩
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/332
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
#define sq(i) (i*i)

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
    ll A, B; cin >> A >> B;

    vector<int> divisor((int)sqrt(B)+1, 1);
    vector<int> AtoB(B-A+1, 1);

    divisor[0] = divisor[1] = 0;
    for (ll i = 2; sq(i) <= B; i++) {
        if (!divisor[i]) continue;

        // sqrt(B)までの素数を列挙
        for (ll j = 2; sq(i*j) <= B; j++) {
            // printf("i:%lld, j:%lld, i*j:%lld, sq(i*j):%lld\n", i, j, i*j, sq(i*j));
            divisor[i*j] = 0;
        }

        // AtoB内の合成数を削除
        // A以上で最小のiの倍数: i*((A-1)/i) + i
        for (ll n = i*((A-1)/i)+i; n <= B; n += i) {
            // nが素数のとき
            if (n == i) continue;
            int j = n - A;
            if (0 <= j && j <= B-A) {
                AtoB[j] = 0;
            }
        }
    }

    // print_vector(divisor);
    print_vector(AtoB);
    cout << accumulate(ALL(AtoB), 0) << endl;
}
