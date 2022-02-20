#include <iostream>
#include <vector>
using ll = long long;
using namespace std;

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int main()
{
    int N, K;
    cin >> N >> K;
    vector<int> a(N, 0);
    vector<int> b(N, 0);
    for (int i = 0; i < K; i++)
    {
        char c;
        int k;
        cin >> c >> k;
        k--;
        b[k] = 1;
        for (int j = 0; j < N; j++)
        {
            if (c == 'L' && k <= j)
                a[j]++;
            if (c == 'R' && j <= k)
                a[j]++;
        }
    }

    print_vector(a);
    print_vector(b);

    const ll MOD = 998244353;
    ll ans = 1;
    for (int i = 0; i < N; i++)
        if (!b[i])
            ans = ans * a[i] % MOD;

    cout << ans << endl;
    return 0;
}
