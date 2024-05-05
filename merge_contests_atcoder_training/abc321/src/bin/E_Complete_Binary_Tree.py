#         E - Complete Binary Tree        
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc321/tasks/abc321_e
# ----------------------------------------

T = int(input())

for _ in range(T):
    N, X, K = map(int, input().split())

    if K > N.bit_length() << 1:
        print(0)
        continue

    ans = 0

    # 子について考える
    left = X << K
    right = (X + 1) << K
    ans += max(0, min(right, N + 1) - left)

    prev = X
    while True:
        # 親に移動
        X >>= 1
        if X == 0:
            break
        K -= 1
        if K == 0:
            ans += 1
            break
        elif K > 0:
            if X << 1 == prev:
                # 右の子を探索
                rc = (X << 1) + 1
                left = rc << K - 1
                right = (rc + 1) << K - 1
                ans += max(0, min(right, N + 1) - left)
            else:
                # 左の子を探索
                lc = X << 1
                left = lc << K - 1
                right = (lc + 1) << K - 1
                ans += max(0, min(right, N + 1) - left)
        prev = X
    
    print(ans)
