# AC


K = int(input())
A, B = map(int, input().split())

def dec(n, k):
    str_n = str(n)
    d = len(str_n)
    res = sum( int(str_n[i]) * k**(d-i-1) for i in range(d) )
    return res

print(dec(A, K) * dec(B, K))