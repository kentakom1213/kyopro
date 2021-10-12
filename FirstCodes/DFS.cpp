// AC

#include <bits/stdc++.h>
using namespace std;


void search(const vector<string>& Field, vector<vector<bool>>& Reached, int x, int y, bool& can_goal) {

  if (x < 0 || y < 0 || x > Field.size() - 1 || y > Field.at(0).size() - 1 || Field.at(x).at(y) == '#') {
    can_goal = can_goal || false;
    return;
  }

  char state = Field.at(x).at(y);
  //printf("(%d, %d) %c\n", x ,y, state);

  if (Reached.at(x).at(y)) {
    can_goal = can_goal || false;
    return;
  }

  if (state == 'g') {
    can_goal = can_goal || true;
    return;
  }

  Reached.at(x).at(y) = true;

  search(Field, Reached, x+1, y, can_goal); // right
  search(Field, Reached, x, y-1, can_goal); // down
  search(Field, Reached, x-1, y, can_goal); // left
  search(Field, Reached, x, y+1, can_goal); // up

}



int main() {
  int H, W;
  cin >> H >> W;
  
  vector<vector<bool>> Reached(H, vector<bool>(W, false));
  vector<string> Field(H);

  int start_x, start_y;

  for (int i = 0; i < H; i++) {
    string line;
    cin >> line;
    Field.at(i) = line;

    if (line.find('s') != -1) {
      start_x = i;
      start_y = line.find('s');
    }

  }

  bool exist_route = false;

  search(Field, Reached, start_x, start_y, exist_route);

  if (exist_route) cout << "Yes" << endl;
  else cout << "No" << endl;

}
