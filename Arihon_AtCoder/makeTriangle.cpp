// AC

#include <bits/stdc++.h>
using namespace std;

bool canBeTriangle(int a, int b, int c) {
  // 全ての辺が同じ長さの時は条件を満たさないので除外
  if (a == b || b == c || c == a) {
    return false;
  } else if (a + b > c && b + c > a && c +  a > b) {
    return true;
  } else {
    return false;
  }
}

int testEdges(vector<int> edges) {
  int count = 0;
  int N = edges.size();
  // 全探索
  for (int i = 0; i < N-2; i++) {
    for (int j = i + 1; j < N-1; j++) {
      for (int k = j + 1; k < N; k++) {
        //printf("(%d, %d, %d)\n", i, j, k);
        if (canBeTriangle(edges.at(i), edges.at(j), edges.at(k))) {
          count++;
        }
      }
    }
  }
  return count;
}


void print_vector(vector<int> vec) {
  cout << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    cout << vec.at(i) << " ";
  }
  cout << "]" << endl;
}

int main() {
  int N;
  cin >> N;
  vector<int> edges(N);

  for (int i = 0; i < N; i++) {
    cin >> edges.at(i);
  }

  //print_vector(edges);
  cout << testEdges(edges) << endl;
}
