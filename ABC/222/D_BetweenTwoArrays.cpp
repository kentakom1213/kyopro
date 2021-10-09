

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

const int MOD = 998244353;

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
    vector<int> A(N), B(N);
    for (int i = 0; i < N; i++) cin >> A[i];
    for (int i = 0; i < N; i++) cin >> B[i];

    const int MAX = 10;
    vector<int> dp(MAX, 0), S(MAX, 0);
    dp[A[0]]++;

    for (int i = 0; i < N; i++) {
        for (int j = A[i]; j <= B[i]; j++) {
            dp[j] = (dp[j] + S[j-1]) % MOD;
            
            // 累積和
            S[j] = S[j-1] + dp[j];
        }
        print_vector(dp);
        print_vector(S);
    }

    cout << S[N] << endl;
}