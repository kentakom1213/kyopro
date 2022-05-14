
#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

bool arr[1010101];

template<class T>
void print_set(set<T> S) {
    cout << "{ ";
    for (auto val : S) {
        cout << val << ", "[val == *(--S.end())];
    }
    cout << "}" << endl;
}

int main() {
    ll W;
    cin >> W;

    arr[1] = arr[2] = 1;

    set<ll> st;
    st.insert(1);
    st.insert(2);

    int i = 1;
    while (i <= W) {
        for (auto x : st) {
            arr[i+x] = 1;
            for (auto y : st) {
                if (x == y) continue;
                arr[i+x+y] = 1;
            }
        } 
        while (arr[i] == 1 && i <= W) i++;
        arr[i] = 1;
        st.insert(i);
    }

    cout << st.size() << endl;
    for (auto x : st) cout << x << " "; cout << endl;
}