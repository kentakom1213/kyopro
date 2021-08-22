// AC

#include <bits/stdc++.h>
using namespace std;

int search_combinations(int K, int S);

int main() {
  int K, S;
  cin >> K >> S;

  cout << search_combinations(K, S) << endl;
}

int search_combinations(int K, int S) {
  int count = 0;

  for (int x = 0; x <= K; x++) {
    for (int y = 0; y <= K; y++) {
      int z = S - (x + y);

      if (0 <= z && z <= K) {
        //printf("(%d, %d, %d)\n", x, y, z);
        count++;
      }
    }
  }
  return count;
}