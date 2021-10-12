#include <bits/stdc++.h>
using namespace std;

// solve
int main() {
  int R, C;
  cin >> R >> C;

  pair<int, int> start;
  pair<int, int> goal;
  cin >> start.first >> start.second;
  cin >> goal.first >> goal.second;

  vector<string> field(R);
  for (int i = 0; i < R; i++) {
    cin >> field[i];
  }

  //wfs
  queue<pair<int, int>> pathes;
  
}