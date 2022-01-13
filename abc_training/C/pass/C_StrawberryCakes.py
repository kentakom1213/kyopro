#        C - Strawberry Cakes
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/ddcc2020-qual/tasks/ddcc2020_qual_c
# ----------------------------------------

# 長方形あるある:対角の2頂点を決めれば決定する
# 貪欲にいけば多分大丈夫

# なかなかうまく行かない
# if __name__ == "__main__":
#     H, W, K = map(int, input().split())
#     field = [input() for _ in range(H)]

#     # どの行のberryを使えば良いか

#     # row_berry = [-1] * H
#     # for r in range(H):
#     #     if field[r].count("#"):
#     #         row_berry[0] = r
#     #         break

#     # for r in range(1, H):
#     #     berry = field[r].count("#")

#     #     if berry == 0:
#     #         row_berry[r] = row_berry[r-1]
#     #     else:
#     #         row_berry[r] = r
    
#     # 行毎に考える
#     row_berry = [row.count("#") for row in field]
#     res = [[-1] * W for _ in range(H)]  # 答えを保存する配列

#     berry_number = 1
#     for r in range(H):
#         # 行にberryが存在しない場合、埋める
#         if row_berry[r] == 0:
#             for c in range(W):
#                 res[r][c] = berry_number
        
#         # 行にberryが1つだけ存在する場合、埋めて更新
#         elif row_berry[r] == 1:
#             for c in range(W):
#                 res[r][c] = berry_number
#             berry_number += 1
        
#         # 行に複数berryが存在する場合
#         else:
#             for c in range(W):
#                 if field[r][c] == ".":
#                     res[r][c] = berry_number
#                 else:
#                     res[r][c] = berry_number
#                     berry_number += 1


#     # 出力
#     for r in res:
#         print(*r)