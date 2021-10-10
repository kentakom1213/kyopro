

class Mino:
    def __init__(self, n):
        self.number = n
        *self.size, self.C = map(int, input().split())
        self.shape = [input() for _ in range(self.size[0])]
    
    def __repr__(self):
        return "\n".join(self.shape)

class Field:
    def __init__(self, N, K):
        self.field = [[0] * N for _ in range(N)]
        self.points = [tuple(map(int, input().split())) for _ in range(K)]
        self.minos = []

    def add_mino(self, mino, x, y):
        self.minos.append((mino, x, y))

    def show(self):
        print(len(self.minos))
        for mino, x, y in self.minos:
            print(mino.number + 1, x, y)

if __name__ == "__main__":

    N, K, B = map(int, input().split())
    f = Field(N, K)
    minos = [Mino(i) for i in range(B)]

    for x, y in f.points:
        f.add_mino(minos[0], x, y)

    f.show()