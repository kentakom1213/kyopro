//                正規表現 3-3
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/295

// AC
// ----------------------------------------

#include <iostream>
#include <regex>
using namespace std;

int main() {
    string S; cin >> S;
    regex reg{R"(^\d{3}\-\d{4}$)"};
    smatch m;

    bool search = regex_search(S, m, reg);
    cout << (search ? "Yes" : "No") << endl;
}