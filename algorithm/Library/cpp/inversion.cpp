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

    long long sum(int i) {
        long long res = 0;
        while (i > 0) {
            res += arr[i];
            i -= i & -i;
        }
        return res;
    }
};

template <class T>
long long inverse_number(const vector<T>& perm) {
    int n = perm.size();
    long long sup = *max_element(perm.begin(), perm.end());
    BIT bit(sup);
    long long res = 0;
    for (int i = n-1; i >= 0; i--) {
        res += bit.sum(perm[i]);
        bit.add(perm[i], 1);
    }
    return res;
}