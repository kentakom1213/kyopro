S, T = input(), input()
print("Yes" if len(S) <= len(T) and T[:len(S)] == S else "No")