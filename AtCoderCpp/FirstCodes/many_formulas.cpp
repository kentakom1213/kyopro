// AC

#include <bits/stdc++.h>
using namespace std;

int main() {
  string number;
  cin >> number;
  const int N = number.size() - 1;

  long long sum = 0;
  for (int tmp = 0; tmp < (1 << N); tmp++) {
    bitset<10> sep_bits(tmp);
    // 区切りを定義
    vector<int> seps(1, 0);
    for (int i = 0; i < 10; i++) {
      if (sep_bits.test(i)) seps.push_back(i+1);
    }
    seps.push_back(N+1);

    // 和を計算
    //vector<int> nums; // デバッグ用
    for (int i = 0; i < seps.size() - 1; i++) {
      long long separated = stoll( number.substr(seps.at(i), seps.at(i+1) - seps.at(i)).c_str() );
      //nums.push_back(separated); // デバッグ用
      sum += separated;
    }
    //print_vector(nums); // デバッグ用
  }
  cout << sum << endl;
}