// AC

#include <bits/stdc++.h>
using namespace std;

int ctoi(char i) {
  return i - '0';
}


int main() {
  string digits;
  cin >> digits;
  
  for (int tmp = 0; tmp < (1 << 3); tmp++) {
    bitset<3> operate(tmp);

    int sum = ctoi(digits.at(0));
    string formula;
    formula.push_back(digits.at(0)); // formula
    for (int i = 0; i < 3; i++) {
      if (operate.test(i)) {
        sum += ctoi(digits.at(i+1));
        formula.push_back('+'); // formula
        formula.push_back(digits.at(i+1)); // formula
      }
      else {
        sum -= ctoi(digits.at(i+1));
        formula.push_back('-'); // formula
        formula.push_back(digits.at(i+1)); // formula
      }
    }
    if (sum == 7) {
      cout << formula << "=7" << endl;
      return 0;
    }
    //cout << formula << "=" << sum << endl;
  }
}