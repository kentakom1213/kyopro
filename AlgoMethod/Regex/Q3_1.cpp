//                正規表現 3-1
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/293

// AC
// ----------------------------------------

#include <iostream>
#include <regex>
using namespace std;

int main() {
    string S; cin >> S;
    regex reg{R"(\d+)"};
    smatch m;

    bool search = regex_search(S, m, reg);
    cout << (search ? "Yes" : "No") << endl;
}