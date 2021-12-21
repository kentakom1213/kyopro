
# Aを4つの区間に区切る: 100^3


from collections import deque

def get_ans():
    n = int(input())
    A = list(map(int, input().split()))

    


if __name__ == "__main__":
    t = int(input())
    res = [0] * t
    for i in range(t):
        res[i] = get_ans()
    
    print(*res, sep="\n")