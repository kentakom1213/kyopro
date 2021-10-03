//                正規表現 3-4
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/296

// AC
// ----------------------------------------

#include <iostream>
#include <regex>
using namespace std;

int main() {
    string S; cin >> S;
    regex reg{R"(^[a-zA-Z0-9]+@[a-zA-Z0-9]+\.[a-zA-Z0-9]{1,4}$)"};
    smatch m;

    bool search = regex_search(S, m, reg);
    cout << (search ? "Yes" : "No") << endl;
}