N, M, K = map(int, input().split())
A = [0] * M
R = [0] * M

for i in range(M):
    c, *a, r = input().split()
    for aa in a:
        flag = int(aa) - 1
        A[i] |= 1 << flag
    R[i] = r == "o"

# bit全探索
ans = 0

for S in range(1 << N):
    isok = True

    for i in range(M):
        cnt = (S & A[i]).bit_count()
        
        isok &= (cnt >= K) == R[i]

    ans += isok

print(ans)
