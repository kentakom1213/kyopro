
# AC

N, P = map(int, input().split())
A = list(map(int, input().split()))

ok = list( filter(lambda x: x<P, A) )

print(len(ok))