
#include <iostream>
#include <map>
using namespace std;

template<class T, class U>
void print_map(map<T, U> dict) {
    cerr << "{\n";
    for (auto &[a, b] : dict) {
        cerr << "   {" << a << ", " << b << "},\n";
    }
    cerr << "}" << endl;
}


int main() {
    map<int, int> test;
    test[1] = 5;
    test[2] = 8;
    test[9] = 21;
    test[4] = 3;

    print_map(test);
}
