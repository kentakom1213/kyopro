
MOD = 998244353

def main():
    N = int(input())
    ans = (pow(2, N, MOD) - 2) % MOD

    for i in range(2, N+1):
        cnt = N // i
        ans -= pow(2, cnt-1)
        ans = (ans + MOD) % MOD
    
    return (ans + MOD) % MOD


T = int(input())
for _ in range(T):
    print(main())
