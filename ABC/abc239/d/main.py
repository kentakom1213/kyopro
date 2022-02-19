
A, B, C, D = map(int, input().split())

def is_prime(n):
    if n == 1:
        return False
    if n == 2:
        return True
    for i in range(2, 1-int(-pow(n,0.5)//1)):
        if n % i == 0:
            return False
    return True

ans = True
for i in range(A, B+1):
    isOK = False
    for j in range(C, D+1):
        isOK |= is_prime(i+j)
        # print(i+j, is_prime(i+j))
    ans &= isOK
    
print("Aoki" if ans else "Takahashi")
