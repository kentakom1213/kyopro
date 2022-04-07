/*
# Ants (POJ)
*/

#include <iostream>
#include <vector>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int> > name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;

int main() {
    int T; cin >> T;
    while (T--) {
        ll L, N; cin >> L >> N;
        vector<ll> X(N);
        rep(i, N) cin >> X[i];

        ll min_time = 0;
        rep(i, N) {
            min_time = max(min_time, min(X[i], L - X[i]));
        }

        ll max_time = 0;
        rep(i, N) {
            max_time = max(max_time, max(X[i], L - X[i]));
        }

        cout << min_time << " " << max_time << endl;
    }

    return 0;
}