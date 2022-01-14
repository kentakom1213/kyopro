#                 Vector
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP2_1_A&lang=ja

# AC
# ----------------------------------------

class Vector:
    def __init__(self, size):
        self.size = size
        self.tail = 0
        self.arr = [0] * size

    def pushBack(self, x):
        self.arr[self.tail] = x
        self.tail += 1
    
    def randomAccess(self, p):
        return self.arr[p]

    def popBack(self):
        self.tail -= 1


if __name__ == "__main__":
    Q = int(input())
    queries = [tuple(map(int, input().split())) for _ in range(Q)]

    vec = Vector(Q)

    for q in queries:
        if q[0] == 0:
            vec.pushBack(q[1])
        elif q[0] == 1:
            val = vec.randomAccess(q[1])
            print(val)
        else:
            vec.popBack()
