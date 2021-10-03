//                正規表現 2-3
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/340

// AC
// ----------------------------------------

#include <iostream>
#include <regex>
using namespace std;

int main() {
    string S; cin >> S;
    regex reg{R"(^[a-z]+(\-[a-z]+)*$)"};
    smatch m;

    bool search = regex_search(S, m, reg);
    cout << (search ? "Yes" : "No") << endl;
}