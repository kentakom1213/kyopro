H, W = map(int, input().split())
S = [input() for _ in range(H)]
snuke = "snuke"

# # 横
# for i in range(H):
#     for j in range(W):
#         if S[i][j:j+5] == snuke:
#             for k in range(5):
#                 print(i + 1, j + k + 1)
#             exit()

#         if S[i][j:j+5] == snuke[::-1]:
#             for k in range(5):
#                 print(i + 1, j - k + 5)

# # 横
# for j in range(W):
#     for i in range(H):
#         tmp = ""
#         ans = []
#         for k in range(5):
#             if i + k >= H:
#                 break
#             tmp += S[i+k][j]
#             ans.append((i + k + 1, j + 1))
#         if tmp == snuke:
#             for x, y in ans:
#                 print(x, y)
#                 exit()
#         # 逆向き
