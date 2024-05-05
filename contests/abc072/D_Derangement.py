#             D - Derangemwnt
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc072/tasks/arc082_b

# 参考
# https://img.atcoder.jp/arc082/editorial.pdf
# ----------------------------------------


from typing import no_type_check


if __name__ == "__main__":
    N = int(input())
    P = list(map(int, input().split()))

    isnot_OK = [0] * N
    for i, p in enumerate(P):
        isnot_OK[i] = i+1 == p

    cnt = 0
    while any(isnot_OK):
        for i in range(N-1):
            if isnot_OK[i] and isnot_OK[i+1]:
                isnot_OK[i] = 0
                isnot_OK[i+1] = 0
                cnt += 1
            elif (not isnot_OK[i]) and isnot_OK[i+1]:
                isnot_OK[i] = 0
                isnot_OK[i+1] = 0
                cnt += 1
    
    print(cnt)