#        C - Dubious Document 2
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc076/tasks/abc076_c

# インデックスのミス怖すぎる
# なるべくシンプルなコードで

# AC
# ----------------------------------------

# 全ての組み合わせを検索してみる
# N = len(S), M = len(T) とするとき
# O(NM)

S, T = input(), input()
N, M = len(S), len(T)

# 文字列検索
ok = []
for i in range(N-M+1):
    j = 0
    while j < M and (S[i+j] == "?" or S[i+j] == T[j]):
        j += 1

    # matchしている場合
    if j == M:
        res = S[:i] + T + S[i+M:]
        res = res.replace("?", "a")
        ok.append(res)
        
if ok:
    ok.sort()
    print(ok[0])
else:
    print("UNRESTORABLE")
