#include <iostream>
#include <utility>
#include <string>

std::pair<int, std::string> f()
{
return {3, "Hello"};
}

int main() {

    // 関数f()の戻り値である整数と文字列の組を分解する。
    // pairのfirstをid変数に代入し、secondをmessage変数に代入する。
    // id変数の型はfirstの型(int)となり、message変数の型はsecondの型(string)となる。
    auto [id, message] = f();

    std::cout << id << std::endl;      // 「3」が出力される
    std::cout << message << std::endl; // 「Hello」が出力される
}