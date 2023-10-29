N = int(input())

a = 1
while a <= N:
    b = 1
    while b <= N:
        if a * b == N:
            print("Yes")
            exit()
        b *= 3
    a *= 2 

print("No")
