
K = int(input())

bin_ = bin(K)[2:]
res = bin_.replace("1", "2")

print(res)
