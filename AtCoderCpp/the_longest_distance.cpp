// AC

#include <bits/stdc++.h>
using namespace std;

void print_pair(pair<int, int> p);
int squere(int x);
int squared_distance(pair<int, int> a, pair<int, int> b);
double search_longest(vector<pair<int, int>> &points);

int main() {
  int N;
  cin >> N;
  vector<pair<int, int>> points;

  for (int i = 0; i < N; i++) {
    pair<int, int> a_point;
    cin >> a_point.first >> a_point.second;
    points.push_back(a_point);
  }

  double longest = search_longest(points);
  cout << longest << endl;

}

void print_pair(pair<int, int> p) {
  printf("(%d, %d)\n", p.first, p.second);
}

int squere(int x) {
  return x * x;
}

int squared_distance(pair<int, int> a, pair<int, int> b) {
  int squered = squere(a.first - b.first) + squere(a.second - b.second);
  return squered;
}

double search_longest(vector<pair<int, int>> &points) {
  int max_length = 0;

  // test all combinations
  for (int i = 0; i < points.size(); i++) {
    for (int j = i+1; j < points.size(); j++) {
      int length = squared_distance(points.at(i), points.at(j));

      if (length > max_length) {
        max_length = length;
      }
    }
  }
  return sqrt(max_length);
}
