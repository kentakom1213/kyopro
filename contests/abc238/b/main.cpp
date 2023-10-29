#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; };
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; };

template<class T>
void print_set(set<T> S) {
    cerr << "{ ";
    for (auto val : S) {
        cerr << val << ", "[val == *(--S.end())];
    }
    cerr << "}" << endl;
}

int main() {
    int N; cin >> N;
    vector<int> A(N);
    for(int i = 0; i < N; i++) cin >> A[i];

    int now = 0;
    set<int> sep;
    sep.insert(360);
    for (int x : A) {
        now = (now + x) % 360;
        sep.insert(now);
    }

    // 感覚を求める
    int max_span = 0;
    now = 0;
    for (auto s : sep) {
        chmax(max_span, s - now);
        now = s;
    }

    cout << max_span << endl;
}