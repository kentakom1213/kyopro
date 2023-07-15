

def combinations(my_list, r=None):
    if r == None:
        r = len(my_list)
    if r == 1:
        return [[val] for val in my_list]  # my_listを要素数1の組合せの組にする
    else:
        result = []
        for i, val in enumerate(my_list):
            rest = combinations(my_list[i+1:], r-1)  # (i+1)番目以降の要素を使えばいい
            for rest_perm in rest:
                perm = [val] + rest_perm
                result.append(perm)
        return result


def main():
    # N, T, M = map(int, input().split())
    # A = [0] * M
    # B = [0] * M
    # for i in range(M):
    #     A[i], B[i] = map(int, input().split())

    for comb in combinations(range(10), 10):
        print(comb)

main()
