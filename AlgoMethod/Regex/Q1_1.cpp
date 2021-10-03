//                正規表現 1-1
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/292

// AC
// ----------------------------------------


#include <iostream>
#include <regex>
using namespace std;

int main() {
    // 入力
    string S; cin >> S;
    // 検索する正規表現
    regex reg{R"(algo)"};
    // マッチした文字列情報
    smatch m;

    // マッチするか
    bool search = regex_search(S, m, reg);
    
    cout << (search ? "Yes" : "No") << endl;
}