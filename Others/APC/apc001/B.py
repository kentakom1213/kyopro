
from string import ascii_lowercase


def find_suffix(i, n, arr):
    
    return i

def get_ans():
    N = int(input())
    A = [int(v)-1 for v in input().split()]
    num = [N-a-1 for a in A]

    res = [0] * N
    for i, n in enumerate(num):
        



    return "".join(res)

if __name__ == "__main__":
    t = int(input())
    res = [0] * t
    for i in range(t):
        res[i] = get_ans()
    
    print(*res, sep="\n")