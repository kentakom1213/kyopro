
N = int(input())
if N >= 42:
    N += 1
print(f"AGC{(3-len(str(N)))*'0'}{N}")