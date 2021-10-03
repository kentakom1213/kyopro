//                正規表現 1-3
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/299

// AC
// ----------------------------------------

#include <iostream>
#include <regex>
using namespace std;

int main() {
    string S; cin >> S;
    regex reg{R"(^a{1,5}b{10}c*$)"};
    smatch m;

    bool search = regex_match(S, m, reg);
    cout << (search ? "Yes" : "No") << endl;
}