//        032 - AtCoder Ekiden（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_af
// ----------------------------------------

// 10! = 3628800
// 順列の全探索が可能

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

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

template <typename T>
void print_array(vector<vector<T>>& array) {
    int H = array.size();

    cerr << "{" << endl;
    for (int i = 0; i < H; i++) {
        int W = array.at(i).size();
        cerr << "  {";
        for (int j = 0; j < W; j++) {
            if (j < W - 1) cerr << array.at(i).at(j) << ", ";
            else cerr << array.at(i).at(j);
        }
        cerr << "}," << endl;
    }
    cerr << "}" << endl;
}

int main() {
    int N; cin >> N;
    initArray(A, N, N, 0);
    rep(i, N) rep(j, N) cin >> A[i][j];
    int M; cin >> M;
    initArray(bad_pair, N, N, 0);
    rep(i, M) {
        int x, y; cin >> x >> y;
        x--, y--;
        bad_pair[x][y] = 1;
        bad_pair[y][x] = 1;
    }

    // 順列の全探索
    ll min_time = 1LL << 30;
    vector<int> perm(N); rep(i, N) perm[i] = i;
    do {
        bool isOK = true;
        ll time = A[perm[N-1]][N-1];
        rep(i, N-1) {
            time += A[perm[i]][i];
            // 隣接する要素を比較
            int be = perm[i], af = perm[i+1];
            if (bad_pair[be][af]) isOK = false;
        }
        // タイムを置き換える
        if (isOK) chmin(min_time, time);
    } while (next_permutation(ALL(perm)));

    cout << ((min_time == 1LL << 30) ? -1 : min_time) << endl;
}
