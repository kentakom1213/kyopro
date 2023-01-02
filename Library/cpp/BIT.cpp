#include <iostream>
#include <vector>
using namespace std;

// BIT
struct BIT {
    int size;
    vector<int> arr;

    BIT(int n) : arr(n+1, 0) { size = n; }
    
    void add(int i, int x) {
        while (i <= size) {
            arr[i] += x;
            i += i & -i;
        }
    }

    int sum(int i) {
        int res = 0;
        while (i) {
            res += arr[i];
            i -= i & -i;
        }
        return res;
    }
};

// test
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B&lang=jp
int main() {
    int n, q; cin >> n >> q;
    BIT bit(n);

    while (q--) {
        int com, x, y;
        cin >> com >> x >> y;
        if (com == 0) {
            bit.add(x, y);
        } else {
            cout << bit.sum(y) - bit.sum(x-1) << endl;
        }
    }

    return 0;
}
