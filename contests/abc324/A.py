N = input()

A = list(map(int, input().split()))

print("Yes" if max(A) == min(A) else "No")
