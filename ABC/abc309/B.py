
N = int(input())
A = [input() for _ in range(N)]

# 外周
out = A[0] + "".join(A[i][-1] for i in range(1, N - 1)) + A[N - 1][::-1] + "".join(A[i][0] for i in range(N - 2, 0, -1))

# 外周をずらして格納
after = out[-1] + out[:-1]

top = after[:N]
right = after[N:2*N-2]
bottom = after[2*N-2:3*N-2][::-1]
left = after[3*N-2:][::-1]

print(top)
for i in range(N - 2):
    print(left[i], end="")
    print(A[i+1][1:-1], end="")
    print(right[i])

print(bottom)
