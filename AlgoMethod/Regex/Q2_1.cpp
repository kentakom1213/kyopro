//                正規表現 2-1
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/297
// ----------------------------------------

#include <iostream>
#include <regex>
using namespace std;

int main() {
    string S; cin >> S;
    regex reg{R"(1\+1)"};
    smatch m;

    bool search = regex_search(S, m, reg);
    cout << (search ? "Yes" : "No") << endl;
}