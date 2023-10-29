# シートに覆われている場合

# # 左から貪欲に処理
# cnt = 0
# now = 0
# for x in A:
#     if x <= now <= x+W: # シートに覆われている場合
#         now = x+W
#     else:
#         # シートを追加
#         while not (x <= now <= x+W):
#             now += W
#             cnt += 1

# print(cnt)
