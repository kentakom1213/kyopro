N = int(input())

if abs(N % 5) < abs(N % -5):
    print(N - N % 5)
else:
    print(N - N % -5)
