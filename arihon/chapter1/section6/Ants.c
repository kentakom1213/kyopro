/*
# Ants
*/

#include<stdio.h>
#include<math.h>
typedef long long ll;
#define rep(i, N) for (int i=0; i < (int)N; i++)

int main(void) {
    int T; scanf("%d", &T);
    while (T--) {
        ll L, N; scanf("%lld %lld", &L, &N);
        ll X[N];
        rep(i, N) scanf("%lld", &X[i]);

        ll min_time = 0;
        rep(i, N) {
            chmax(min_time, min(X[i], L - X[i]));
        }

        ll max_time = 0;
        rep(i, N) {
            chmax(max_time, max(X[i], L - X[i]));
        }

        printf("%lld %lld\n", min_time, max_time);
    }

    return 0;
}