#include <bits/stdc++.h>
using namespace std;

void print_vector(vector<int> vec, string begin_str="[ ", string end_str=" ]", string sep_str=" ") {
  cout << begin_str;
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cout << vec.at(i) << sep_str;
    else cout << vec.at(i);
  }
  cout << end_str << endl;
}

void cicle_vector(vector<int> &v, int n) {
  int s = v.size();
  int N = n > 0 ? n % s : n % s + s;  // nをsの正の剰余にする
  for (int i = 0; i < N; i++) {
    int back_element = v.at(0);
    v.erase(v.begin());
    v.push_back(back_element);
  }
}

void move_ants(vector<int> &ants, vector<int> &directions, const int L, const int T) {
  int N = ants.size();
  for (int i = 0; i < N; i++) {
    // 時計まわり
    if (directions.at(i) == 1) {
      ants.at(i) += T;
      // L以上の時はあまりを表示
      ants.at(i) %= L;
    }
    // 反時計まわり 
    else {
      ants.at(i) -= T;
      // 0未満の場合はあまりを表示
      ants.at(i) = (ants.at(i) + L) % L;
    }
  }
  sort(ants.begin(), ants.end());
  
  int counter = 0;
  for (int i = 0; i < N; i++) {
    if (directions.at(i) == 1) counter -= (T - (L - ants.at(i)) - L) / L;
    else counter += (T - (ants.at(i) + 1) - L) / L;
  }
  cicle_vector(ants, counter);
}

int main() {
  
  int N, L, T;
  cin >> N >> L >> T;

  vector<int> ants(N);
  vector<int> directions(N);

  for (int i = 0; i < N; i++) {
    cin >> ants.at(i) >> directions.at(i);
  }
  
  move_ants(ants, directions, L, T);

  print_vector(ants, "", "", "\n");
}