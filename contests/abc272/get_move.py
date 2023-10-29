def get_move(m):
    """ 平方の和がmになる二整数i,jを求める """
    for i in range(-int(-pow(m, 1/2)//1)):
        j = 1
        while i*i + j*j <= m:
            if i*i + j*j == m:
                yield (i, j)
            j += 1

    yield (0, 0)


if __name__ == "__main__":
    n = int(input())
    for i, j in get_move(n):
        print(i, j)