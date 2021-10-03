//                正規表現 1-2
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/298

// AC
// ----------------------------------------

#include <iostream>
#include <regex>
using namespace std;

int main() {
    string S; cin >> S;
    regex reg{R"(^metho+d$)"};
    smatch m;

    bool search = regex_match(S, m, reg);
    cout << (search ? "Yes" : "No") << endl;
}