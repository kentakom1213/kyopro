# C - 列
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc032/tasks/abc032_c

# 参考
# https://qiita.com/drken/items/ecd1a472d3a0e7db8dce

# "連続する"部分列だからLISとは異なる

# AC (解説)
# ----------------------------------------

# N, K = map(int, input().split())
# seq = [int(input()) for _ in range(N)]

# tail, head = 0, 1
# product = seq[tail]
# counter = tmp = 1

# print()
# while tail <= head:
#     # 進めるだけ進む
#     while head < N-1 and product * seq[head] <= K:
#         product *= seq[head]
#         head += 1
#         print(f"+ tail:{tail}, head:{head} {seq[tail:head]}:{seq[head]}, {product}")
    
#     # 後ろを一つ消す
#     product //= seq[tail]
#     tail += 1
#     print(f"- tail:{tail}, head:{head} {seq[tail:head]}, {product}" if tail < N else "")

### 参考

def inchworm(seq, k):
    n = len(seq)
    res = 0
    right = 0
    mul = 1

    if 0 in seq:
        return n

    for left in range(n):  # leftを1づつ増やす
        while right < n and mul * seq[right] <= k:  # rightは増やせるだけ増やす
            mul *= seq[right]
            right += 1
        res = max(res, right - left)

        if left == right:  # 一致している場合は割らない -> 0になる
            right += 1
        else:
            mul //= seq[left]
    
    return res

if __name__ == "__main__":
    N, K = map(int, input().split())
    seq = [int(input()) for _ in range(N)]

    print(inchworm(seq, K))