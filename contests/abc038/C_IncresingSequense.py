#               C - 単調増加
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc038/tasks/abc038_c

# 参考
# https://qiita.com/drken/items/ecd1a472d3a0e7db8dce
# https://qiita.com/ngtkana/items/0b8f619a406d5f5b89de
# ----------------------------------------

def inchworm(seq):
    n = len(seq)
    counter = 0
    right = 0

    for left in range(n):
        while right < n and (right <= left or seq[right-1] < seq[right]):  # ...(i)
            right += 1
        
        counter += right - left

        if left == right:
            right += 1

    return counter

# 意味
# (i)
# 単調増加のとき -> r++
# 


### subtask AC ###
# O(n^2) のアルゴリズム、部分点はもらえる

def all_pattern(seq):
    n = len(seq)
    counter = 0

    for i in range(n):
        for j in range(i+1, n):
            if seq[j-1] < seq[j]:
                counter += 1
            else:
                break
    
    return counter + n




if __name__ == "__main__":
    N = int(input())
    seq = list(map(int, input().split()))

    print(inchworm(seq))
    # print(all_pattern(seq))