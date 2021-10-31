#         055 - Select 5（★2） 
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_bc
# ----------------------------------------

# 100C5 = 75287520 = 10^7.8  # 間にあう？
# ↓案の定TLE

# from itertools import combinations

# if __name__ == "__main__":
#     N, P, Q = map(int, input().split())
#     A = list(map(int, input().split()))

    # res = 0
    # for comb in combinations(A, 5):
    #     mod = 1
    #     for n in comb:
    #         mod = (mod * n) % P
        
    #     if mod == Q:
    #         res += 1
    
#     print(res)

# from itertools import combinations

# # dictで重複を排除
# if __name__ == "__main__":
#     N, P, Q = map(int, input().split())
#     A = list(map(int, input().split()))

#     A_map = {}  # {num, [num % P, A.count(num)]}
#     for a in A:
#         if a in A_map:
#             A_map[a][1] += 1
#         else:
#             A_map[a] = [a % P, 1]
    
#     res = 0
#     for comb in combinations(A_map.values(), 5):

#         mod = 1
#         cnt = 1
#         for a_mod, a_cnt in comb:
#             mod = (mod * a_mod) % P
#             cnt *= a_cnt
        
#         if mod == Q:
#             res += cnt
    
#     print(res)


# 解説
# O(n^5)だけど n^5 / 120 であるため、うまくいくらしい
# じゃあ最初からPython無理じゃん!!
# O(n^5) の壁は突破しにくいらしいので大人しくC++をつかおう